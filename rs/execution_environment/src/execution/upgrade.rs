// This module defines how the `install_code` IC method in mode
// `upgrade` is executed.
// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-install_code
// and https://internetcomputer.org/docs/current/references/ic-interface-spec/#system-api-upgrades
use crate::canister_manager::{
    DtsInstallCodeResult, InstallCodeContext, PausedInstallCodeExecution,
};
use crate::execution::common::update_round_limits;
use crate::execution::install_code::{
    canister_layout, finish_err, InstallCodeHelper, OriginalContext, PausedInstallCodeHelper,
    StableMemoryHandling,
};
use crate::execution_environment::{RoundContext, RoundLimits};
use ic_base_types::PrincipalId;
use ic_embedders::wasm_executor::{CanisterStateChanges, PausedWasmExecution, WasmExecutionResult};
use ic_interfaces::execution_environment::{HypervisorError, WasmExecutionOutput};
use ic_interfaces::messages::RequestOrIngress;
use ic_logger::{info, warn, ReplicaLogger};
use ic_replicated_state::{CanisterState, SystemState};
use ic_system_api::ApiType;
use ic_types::funds::Cycles;
use ic_types::methods::{FuncRef, SystemMethod, WasmMethod};

#[cfg(test)]
mod tests;

/// Performs a canister upgrade. The algorithm consists of six stages:
/// - Stage 0: validate input.
/// - Stage 1: invoke `canister_pre_upgrade()` (if present) using the old code.
/// - Stage 2: create a new execution state based on the new Wasm code.
/// - Stage 3: invoke the `start()` method (if present).
/// - Stage 4: invoke the `canister_post_upgrade()` method (if present).
/// - Stage 5: finalize execution and refund execution cycles.
///
/// With deterministic time slicing stages 2, 3, and 4 may require multiple
/// rounds to complete. In order to support that, the algorithm is implemented
/// as a state machine:
/// ```text
/// [begin]
///   │
///   ▼
/// [validate input]
///   │
///   │                                   exceeded slice
///   ▼                                 instruction limit
/// [execute canister_pre_upgrade()] ─────────────────────────► PausedPreUpgradeExecution
///   │                                                           │    │          ▲
///   │                                                           │    └──────────┘
///   │            finished execution                             │    exceeded slice
///   │◄──────────────────────────────────────────────────────────┘   instruction limit
///   │
///   │
///   │
///   ▼
/// [create new execution state]
///   │
///   │
///   │                         exceeded slice
///   ▼                       instruction limit
/// [execute start()]─────────────────────────────────► PausedStartExecutionDuringUpgrade
///   │                                                   │       │         ▲
///   │                                                   │       └─────────┘
///   │             finished execution                    │      exceeded slice
///   │◄──────────────────────────────────────────────────┘     instruction limit
///   │
///   │
///   │                                    exceeded slice
///   ▼                                  instruction limit
/// [execute canister_post_upgrade()] ───────────────────────► PausedPostUpgradeExecution
///   │                                                          │    │          ▲
///   │                                                          │    └──────────┘
///   │            finished execution                            │    exceeded slice
///   │◄─────────────────────────────────────────────────────────┘   instruction limit
///   │
///   │
/// [finalize execution and refund execution cycles]
///   │
///   │
///   ▼
/// [end]
///```
#[allow(clippy::too_many_arguments)]
pub(crate) fn execute_upgrade(
    context: InstallCodeContext,
    clean_canister: CanisterState,
    original: OriginalContext,
    round: RoundContext,
    round_limits: &mut RoundLimits,
) -> DtsInstallCodeResult {
    let mut helper = InstallCodeHelper::new(&clean_canister, &original);

    // Stage 0: validate input.
    if let Err(err) = helper.validate_input(&original, round_limits) {
        return finish_err(
            clean_canister,
            helper.instructions_left(),
            original,
            round,
            err,
        );
    }

    // Stage 1: invoke `canister_pre_upgrade()` (if present) using the old code.

    // Validate that the Wasm module is present.
    let canister_id = context.canister_id;
    let execution_state = match helper.canister().execution_state.as_ref() {
        Some(es) => es,
        None => {
            return finish_err(
                clean_canister,
                helper.instructions_left(),
                original,
                round,
                (canister_id, HypervisorError::WasmModuleNotFound).into(),
            );
        }
    };

    let method = WasmMethod::System(SystemMethod::CanisterPreUpgrade);
    if !execution_state.exports_method(&method) {
        // If the Wasm module does not export the method, then this execution
        // succeeds as a no-op.
        upgrade_stage_2_and_3a_create_execution_state_and_call_start(
            context,
            clean_canister,
            helper,
            original,
            round,
            round_limits,
        )
    } else {
        let wasm_execution_result = round.hypervisor.execute_dts(
            ApiType::pre_upgrade(original.time, context.sender),
            execution_state,
            &helper.canister().system_state,
            helper.canister_memory_usage(),
            helper.execution_parameters().clone(),
            FuncRef::Method(method),
            round_limits,
            round.network_topology,
        );

        match wasm_execution_result {
            WasmExecutionResult::Finished(slice, output, canister_state_changes) => {
                update_round_limits(round_limits, &slice);
                upgrade_stage_1_process_pre_upgrade_result(
                    canister_state_changes,
                    output,
                    context,
                    clean_canister,
                    helper,
                    original,
                    round,
                    round_limits,
                )
            }
            WasmExecutionResult::Paused(slice, paused_wasm_execution) => {
                info!(
                    round.log,
                    "[DTS] Pausing (canister_pre_upgrade) execution of canister {} after {} instructions.",
                    clean_canister.canister_id(),
                    slice.executed_instructions,
                );
                update_round_limits(round_limits, &slice);
                let paused_execution = Box::new(PausedPreUpgradeExecution {
                    paused_wasm_execution,
                    paused_helper: helper.pause(),
                    context,
                    original,
                });
                DtsInstallCodeResult::Paused {
                    canister: clean_canister,
                    paused_execution,
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn upgrade_stage_1_process_pre_upgrade_result(
    canister_state_changes: Option<CanisterStateChanges>,
    output: WasmExecutionOutput,
    context: InstallCodeContext,
    clean_canister: CanisterState,
    mut helper: InstallCodeHelper,
    original: OriginalContext,
    round: RoundContext,
    round_limits: &mut RoundLimits,
) -> DtsInstallCodeResult {
    let result = helper.handle_wasm_execution(canister_state_changes, output, &original, &round);

    info!(
        round.log,
        "Executing (canister_pre_upgrade) on canister {} consumed {} instructions.  {} instructions are left.",
        helper.canister().canister_id(),
        helper.instructions_consumed(),
        helper.instructions_left(),
    );

    if let Err(err) = result {
        let instructions_left = helper.instructions_left();
        return finish_err(clean_canister, instructions_left, original, round, err);
    }

    upgrade_stage_2_and_3a_create_execution_state_and_call_start(
        context,
        clean_canister,
        helper,
        original,
        round,
        round_limits,
    )
}

#[allow(clippy::too_many_arguments)]
fn upgrade_stage_2_and_3a_create_execution_state_and_call_start(
    context: InstallCodeContext,
    clean_canister: CanisterState,
    mut helper: InstallCodeHelper,
    original: OriginalContext,
    round: RoundContext,
    round_limits: &mut RoundLimits,
) -> DtsInstallCodeResult {
    let canister_id = helper.canister().canister_id();
    // Stage 2: create a new execution state based on the new Wasm code.
    // Replace the execution state of the canister with a new execution state, but
    // persist the stable memory (if it exists).
    let layout = canister_layout(&original.canister_layout_path, &canister_id);
    let (instructions_from_compilation, result) = round.hypervisor.create_execution_state(
        context.wasm_module,
        layout.raw_path(),
        canister_id,
        round_limits,
        original.compilation_cost_handling,
    );

    if let Err(err) = helper.replace_execution_state_and_allocations(
        instructions_from_compilation,
        result,
        StableMemoryHandling::Keep,
        &original,
    ) {
        let instructions_left = helper.instructions_left();
        return finish_err(clean_canister, instructions_left, original, round, err);
    }

    // Stage 3: invoke the `start()` method (if present).
    let method = WasmMethod::System(SystemMethod::CanisterStart);

    // The execution state is present because we just put it there.
    let execution_state = helper.canister().execution_state.as_ref().unwrap();
    if !execution_state.exports_method(&method) {
        // If the Wasm module does not export the method, then this execution
        // succeeds as a no-op.
        upgrade_stage_4a_call_post_upgrade(
            context.sender,
            context.arg,
            clean_canister,
            helper,
            original,
            round,
            round_limits,
        )
    } else {
        let wasm_execution_result = round.hypervisor.execute_dts(
            ApiType::start(),
            execution_state,
            &SystemState::new_for_start(canister_id),
            helper.canister_memory_usage(),
            helper.execution_parameters().clone(),
            FuncRef::Method(method),
            round_limits,
            round.network_topology,
        );

        match wasm_execution_result {
            WasmExecutionResult::Finished(slice, output, canister_state_changes) => {
                update_round_limits(round_limits, &slice);
                upgrade_stage_3b_process_start_result(
                    canister_state_changes,
                    output,
                    context.sender,
                    context.arg,
                    clean_canister,
                    helper,
                    original,
                    round,
                    round_limits,
                )
            }
            WasmExecutionResult::Paused(slice, paused_wasm_execution) => {
                info!(
                    round.log,
                    "[DTS] Pausing (start) execution of canister {} after {} instructions.",
                    clean_canister.canister_id(),
                    slice.executed_instructions,
                );
                update_round_limits(round_limits, &slice);
                let paused_execution = Box::new(PausedStartExecutionDuringUpgrade {
                    paused_wasm_execution,
                    paused_helper: helper.pause(),
                    context_sender: context.sender,
                    context_arg: context.arg,
                    original,
                });
                DtsInstallCodeResult::Paused {
                    canister: clean_canister,
                    paused_execution,
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn upgrade_stage_3b_process_start_result(
    canister_state_changes: Option<CanisterStateChanges>,
    output: WasmExecutionOutput,
    context_sender: PrincipalId,
    context_arg: Vec<u8>,
    clean_canister: CanisterState,
    mut helper: InstallCodeHelper,
    original: OriginalContext,
    round: RoundContext,
    round_limits: &mut RoundLimits,
) -> DtsInstallCodeResult {
    let result = helper.handle_wasm_execution(canister_state_changes, output, &original, &round);

    info!(
        round.log,
        "Executing (start) on canister {} consumed {} instructions.  {} instructions are left.",
        helper.canister().canister_id(),
        helper.instructions_consumed(),
        helper.instructions_left(),
    );

    if let Err(err) = result {
        let instructions_left = helper.instructions_left();
        return finish_err(clean_canister, instructions_left, original, round, err);
    }

    upgrade_stage_4a_call_post_upgrade(
        context_sender,
        context_arg,
        clean_canister,
        helper,
        original,
        round,
        round_limits,
    )
}

#[allow(clippy::too_many_arguments)]
fn upgrade_stage_4a_call_post_upgrade(
    context_sender: PrincipalId,
    context_arg: Vec<u8>,
    clean_canister: CanisterState,
    helper: InstallCodeHelper,
    original: OriginalContext,
    round: RoundContext,
    round_limits: &mut RoundLimits,
) -> DtsInstallCodeResult {
    // Stage 4: invoke the `canister_post_upgrade()` method (if present).

    let method = WasmMethod::System(SystemMethod::CanisterPostUpgrade);

    // The execution state is guaranteed to be present because this function is
    // called after creating a new execution state.
    let execution_state = helper.canister().execution_state.as_ref().unwrap();
    if !execution_state.exports_method(&method) {
        // If the Wasm module does not export the method, then this execution
        // succeeds as a no-op.
        return helper.finish(clean_canister, original, round, round_limits);
    }

    let wasm_execution_result = round.hypervisor.execute_dts(
        ApiType::init(original.time, context_arg, context_sender),
        execution_state,
        &helper.canister().system_state,
        helper.canister_memory_usage(),
        helper.execution_parameters().clone(),
        FuncRef::Method(method),
        round_limits,
        round.network_topology,
    );
    match wasm_execution_result {
        WasmExecutionResult::Finished(slice, output, canister_state_changes) => {
            update_round_limits(round_limits, &slice);
            upgrade_stage_4b_process_post_upgrade_result(
                canister_state_changes,
                output,
                clean_canister,
                helper,
                original,
                round,
                round_limits,
            )
        }
        WasmExecutionResult::Paused(slice, paused_wasm_execution) => {
            info!(
                round.log,
                "[DTS] Pausing (canister_post_upgrade) execution of canister {} after {} instructions.",
                clean_canister.canister_id(),
                slice.executed_instructions,
            );
            update_round_limits(round_limits, &slice);
            let paused_execution = Box::new(PausedPostUpgradeExecution {
                paused_wasm_execution,
                paused_helper: helper.pause(),
                original,
            });
            DtsInstallCodeResult::Paused {
                canister: clean_canister,
                paused_execution,
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn upgrade_stage_4b_process_post_upgrade_result(
    canister_state_changes: Option<CanisterStateChanges>,
    output: WasmExecutionOutput,
    clean_canister: CanisterState,
    mut helper: InstallCodeHelper,
    original: OriginalContext,
    round: RoundContext,
    round_limits: &mut RoundLimits,
) -> DtsInstallCodeResult {
    let result = helper.handle_wasm_execution(canister_state_changes, output, &original, &round);
    info!(
        round.log,
        "Executing (canister_post_upgrade) on canister {} consumed {} instructions.  {} instructions are left.",
        helper.canister().canister_id(),
        helper.instructions_consumed(),
        helper.instructions_left();
    );
    if let Err(err) = result {
        let instructions_left = helper.instructions_left();
        return finish_err(clean_canister, instructions_left, original, round, err);
    }
    helper.finish(clean_canister, original, round, round_limits)
}

/// Struct used to hold necessary information for the
/// deterministic time slicing execution of canister upgrade.
/// Pre upgrade is the first stage of the upgrade procedure.
#[derive(Debug)]
struct PausedPreUpgradeExecution {
    paused_wasm_execution: Box<dyn PausedWasmExecution>,
    paused_helper: PausedInstallCodeHelper,
    context: InstallCodeContext,
    original: OriginalContext,
}

impl PausedInstallCodeExecution for PausedPreUpgradeExecution {
    fn resume(
        self: Box<Self>,
        clean_canister: CanisterState,
        round: RoundContext,
        round_limits: &mut RoundLimits,
    ) -> DtsInstallCodeResult {
        info!(
            round.log,
            "[DTS] Resuming (canister_pre_upgrade) execution of canister {}.",
            clean_canister.canister_id(),
        );
        let helper = match InstallCodeHelper::resume(
            &clean_canister,
            self.paused_helper,
            &self.original,
            &round,
            round_limits,
        ) {
            Ok(helper) => helper,
            Err((err, instructions_left)) => {
                warn!(
                    round.log,
                    "[DTS] Canister {} failed to resume paused (canister_pre_upgrade) execution: {:?}.",
                    clean_canister.canister_id(),
                    err
                );
                self.paused_wasm_execution.abort();
                return finish_err(clean_canister, instructions_left, self.original, round, err);
            }
        };
        let execution_state = helper.canister().execution_state.as_ref().unwrap();
        let wasm_execution_result = self.paused_wasm_execution.resume(execution_state);
        match wasm_execution_result {
            WasmExecutionResult::Finished(slice, output, canister_state_changes) => {
                update_round_limits(round_limits, &slice);
                upgrade_stage_1_process_pre_upgrade_result(
                    canister_state_changes,
                    output,
                    self.context,
                    clean_canister,
                    helper,
                    self.original,
                    round,
                    round_limits,
                )
            }
            WasmExecutionResult::Paused(slice, paused_wasm_execution) => {
                info!(
                    round.log,
                    "[DTS] Pausing (canister_pre_upgrade) execution of canister {} after {} instructions.",
                    clean_canister.canister_id(),
                    slice.executed_instructions,
                );
                update_round_limits(round_limits, &slice);
                let paused_execution = Box::new(PausedPreUpgradeExecution {
                    paused_wasm_execution,
                    paused_helper: helper.pause(),
                    ..*self
                });
                DtsInstallCodeResult::Paused {
                    canister: clean_canister,
                    paused_execution,
                }
            }
        }
    }

    fn abort(self: Box<Self>, log: &ReplicaLogger) -> (RequestOrIngress, Cycles) {
        info!(
            log,
            "[DTS] Aborting (canister_pre_upgrade) execution of canister {}.",
            self.original.canister_id
        );
        self.paused_wasm_execution.abort();
        (
            self.original.message,
            self.original.prepaid_execution_cycles,
        )
    }
}

/// Struct used to hold necessary information for the
/// deterministic time slicing execution of canister upgrade.
/// Start is the second stage of the upgrade procedure.
#[derive(Debug)]
struct PausedStartExecutionDuringUpgrade {
    paused_wasm_execution: Box<dyn PausedWasmExecution>,
    paused_helper: PausedInstallCodeHelper,
    context_sender: PrincipalId,
    context_arg: Vec<u8>,
    original: OriginalContext,
}

impl PausedInstallCodeExecution for PausedStartExecutionDuringUpgrade {
    fn resume(
        self: Box<Self>,
        clean_canister: CanisterState,
        round: RoundContext,
        round_limits: &mut RoundLimits,
    ) -> DtsInstallCodeResult {
        info!(
            round.log,
            "[DTS] Resuming (start) execution of canister {}.",
            clean_canister.canister_id(),
        );
        let helper = match InstallCodeHelper::resume(
            &clean_canister,
            self.paused_helper,
            &self.original,
            &round,
            round_limits,
        ) {
            Ok(helper) => helper,
            Err((err, instructions_left)) => {
                warn!(
                    round.log,
                    "[DTS] Canister {} failed to resume paused (start) execution: {:?}.",
                    clean_canister.canister_id(),
                    err
                );
                self.paused_wasm_execution.abort();
                return finish_err(clean_canister, instructions_left, self.original, round, err);
            }
        };
        let execution_state = helper.canister().execution_state.as_ref().unwrap();
        let wasm_execution_result = self.paused_wasm_execution.resume(execution_state);
        match wasm_execution_result {
            WasmExecutionResult::Finished(slice, output, canister_state_changes) => {
                update_round_limits(round_limits, &slice);
                upgrade_stage_3b_process_start_result(
                    canister_state_changes,
                    output,
                    self.context_sender,
                    self.context_arg,
                    clean_canister,
                    helper,
                    self.original,
                    round,
                    round_limits,
                )
            }
            WasmExecutionResult::Paused(slice, paused_wasm_execution) => {
                info!(
                    round.log,
                    "[DTS] Pausing (start) execution of canister {} after {} instructions.",
                    clean_canister.canister_id(),
                    slice.executed_instructions,
                );
                update_round_limits(round_limits, &slice);
                let paused_execution = Box::new(PausedStartExecutionDuringUpgrade {
                    paused_wasm_execution,
                    paused_helper: helper.pause(),
                    ..*self
                });
                DtsInstallCodeResult::Paused {
                    canister: clean_canister,
                    paused_execution,
                }
            }
        }
    }

    fn abort(self: Box<Self>, log: &ReplicaLogger) -> (RequestOrIngress, Cycles) {
        info!(
            log,
            "[DTS] Aborting (start) execution of canister {}.", self.original.canister_id
        );

        self.paused_wasm_execution.abort();
        (
            self.original.message,
            self.original.prepaid_execution_cycles,
        )
    }
}

/// Struct used to hold necessary information for the
/// deterministic time slicing execution of canister upgrade.
/// Post upgrade is the third stage of the upgrade procedure.
#[derive(Debug)]
struct PausedPostUpgradeExecution {
    paused_wasm_execution: Box<dyn PausedWasmExecution>,
    paused_helper: PausedInstallCodeHelper,
    original: OriginalContext,
}

impl PausedInstallCodeExecution for PausedPostUpgradeExecution {
    fn resume(
        self: Box<Self>,
        clean_canister: CanisterState,
        round: RoundContext,
        round_limits: &mut RoundLimits,
    ) -> DtsInstallCodeResult {
        info!(
            round.log,
            "[DTS] Resuming (canister_post_upgrade) execution of canister {}.",
            clean_canister.canister_id(),
        );
        let helper = match InstallCodeHelper::resume(
            &clean_canister,
            self.paused_helper,
            &self.original,
            &round,
            round_limits,
        ) {
            Ok(helper) => helper,
            Err((err, instructions_left)) => {
                warn!(
                    round.log,
                    "[DTS] Canister {} failed to resume paused (canister_post_upgrade) execution: {:?}.",
                    clean_canister.canister_id(),
                    err
                );
                self.paused_wasm_execution.abort();
                return finish_err(clean_canister, instructions_left, self.original, round, err);
            }
        };
        let execution_state = helper.canister().execution_state.as_ref().unwrap();
        let wasm_execution_result = self.paused_wasm_execution.resume(execution_state);
        match wasm_execution_result {
            WasmExecutionResult::Finished(slice, output, canister_state_changes) => {
                update_round_limits(round_limits, &slice);
                upgrade_stage_4b_process_post_upgrade_result(
                    canister_state_changes,
                    output,
                    clean_canister,
                    helper,
                    self.original,
                    round,
                    round_limits,
                )
            }
            WasmExecutionResult::Paused(slice, paused_wasm_execution) => {
                info!(
                    round.log,
                    "[DTS] Pausing (canister_post_upgrade) execution of canister {} after {} instructions.",
                    clean_canister.canister_id(),
                    slice.executed_instructions,
                );
                update_round_limits(round_limits, &slice);
                let paused_execution = Box::new(PausedPostUpgradeExecution {
                    paused_wasm_execution,
                    paused_helper: helper.pause(),
                    ..*self
                });
                DtsInstallCodeResult::Paused {
                    canister: clean_canister,
                    paused_execution,
                }
            }
        }
    }

    fn abort(self: Box<Self>, log: &ReplicaLogger) -> (RequestOrIngress, Cycles) {
        info!(
            log,
            "[DTS] Aborting (canister_post_upgrade) execution of canister {}.",
            self.original.canister_id,
        );
        self.paused_wasm_execution.abort();
        (self.original.message, Cycles::zero())
    }
}
