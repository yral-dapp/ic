# Proposal to upgrade the ledger suite orchestrator canister to add ckPEPE

Git hash: `7fbb84aad7188d1d5b3e17b170997c29d1598cb8`

New compressed Wasm hash: `9bd512661aba6bd7895d09685f625beca014304b7c1e073e029794d601a86709`

Target canister: `vxkom-oyaaa-aaaar-qafda-cai`

Previous ledger suite orchestrator proposal: https://dashboard.internetcomputer.org/proposal/130395

---

## Motivation

This proposal upgrades the ckERC20 ledger suite orchestrator to add support for [PEPE](https://etherscan.io/token/0x6982508145454ce325ddbe47a25d4ec3d2311933#tokenInfo). Once executed, the twin token ckPEPE will be available on ICP, refer to the [documentation](https://github.com/dfinity/ic/blob/master/rs/ethereum/cketh/docs/ckerc20.adoc) on how to proceed with deposits and withdrawals.

## Upgrade args

```
git fetch
git checkout 7fbb84aad7188d1d5b3e17b170997c29d1598cb8
cd rs/ethereum/ledger-suite-orchestrator
didc encode -d ledger_suite_orchestrator.did -t '(OrchestratorArg)' '(variant { AddErc20Arg = record { contract = record { chain_id = 1; address = "0x6982508145454ce325ddbe47a25d4ec3d2311933" }; ledger_init_arg = record { minting_account = record { owner = principal "sv3dd-oaaaa-aaaar-qacoa-cai" }; fee_collector_account = opt record { owner = principal "sv3dd-oaaaa-aaaar-qacoa-cai"; subaccount = opt blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0f\ee"; }; feature_flags  = opt record { icrc2 = true }; decimals = opt 18; max_memo_length = opt 80; transfer_fee = 1000_000_000_000_000_000_000; token_symbol = "ckPEPE"; token_name = "ckPEPE"; token_logo = "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyB3aWR0aD0iMTIwIiBoZWlnaHQ9IjEyMCIgZmlsbD0ibm9uZSIgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxnIGNsaXAtcGF0aD0idXJsKCNhKSI+PG1hc2sgaWQ9ImIiIHg9IjAiIHk9IjAiIHdpZHRoPSIxMjAiIGhlaWdodD0iMTIwIiBzdHlsZT0ibWFzay10eXBlOmFscGhhIiBtYXNrVW5pdHM9InVzZXJTcGFjZU9uVXNlIj48cGF0aCBkPSJtNjAgMGMzMy4xMzMgMCA2MCAyNi44NjcgNjAgNjBzLTI2Ljg2NyA2MC02MCA2MC02MC0yNi44NjctNjAtNjAgMjYuODY3LTYwIDYwLTYweiIgZmlsbD0iIzA4NDdGNyIvPjwvbWFzaz48ZyBtYXNrPSJ1cmwoI2IpIj48cGF0aCBkPSJtNjAgMGMxNS45NjgtMC4wOTcxOTIgMzEuODIgNi42MDggNDIuOTI3IDE4LjA3MiAxMC42NDcgMTAuODM3IDE2LjkyOCAyNS44NDMgMTcuMDc2IDQxLjA0MyAwLjI2ODY0IDE0Ljk1MS01LjMyNTIgMjkuOTE3LTE1LjM3MyA0MC45OTgtMTAuMjc0IDExLjQ4Mi0yNS4xMDggMTguNzgtNDAuNDk5IDE5Ljc1NS0xNC44MTMgMS4wNTQ5LTI5Ljk2Mi0zLjYyNDUtNDEuNTM2LTEyLjk0NS0xMi40MTYtOS44NTU1LTIwLjY4OC0yNC43OTUtMjIuMjcxLTQwLjU4NS0xLjYyMzktMTUuMDI2IDIuNjY4MS0zMC42MiAxMS44NzgtNDIuNjIgOS44MjcxLTEzIDI1LjEzNS0yMS43NDkgNDEuMzczLTIzLjM3OCAyLjEzMzctMC4yMjcxOSA0LjI3OTYtMC4zNDAzMyA2LjQyNTMtMC4zNDAxNXoiIGZpbGw9IiMzZDgxMzAiLz48cGF0aCBkPSJtNjkuOTcyIDEwLjkwOWMtMy4zOTY5LTEuMzUxMS03LjExOTcgMC4wNDA5NC0xMC4wMTEgMS45MzM0LTEuNTg1IDAuOTU4ODctMy4xMzA2IDIuMDAxMi00LjgzMjEgMi43NDg2LTIuNjQ1OS0zLjc4MTUtNy40MDc1LTUuMzYzOS0xMS44NS01LjU0NTktMy4xODA5LTAuMTYzOTUtNi4zODg3IDAuMDMzNjMtOS41Mjc3IDAuNTQ1NjEtNC45NTAyIDEuNDk3NC04LjQ2OTggNS43NzQ0LTEwLjY3NCAxMC4yNjgtMS4yOTU0IDIuMTUxMS0zLjE3NDkgMy44NjUyLTQuNzAwMyA1Ljg0NTktMy41MzI0IDQuMTY2NS02LjgyNzkgOC42MDQzLTkuMDg0NiAxMy42MDYtNS4xMDYzIDkuNjQ0Mi0xMC45NDEgMTkuMTc0LTEzLjIyNiAyOS45ODItMC42MDM5MSAzLjAzNDUtMC44MjI1MyA2LjEyNzktMS40MzAxIDkuMTYyMS0xLjAxMSA2LjM4MzUtMS4zNjQzIDEyLjg0Ni0yLjIwNzEgMTkuMjUxIDAuMjQ2ODYgNi42MTYgMi44OTM3IDEzLjUyOSA4LjQyNiAxNy40OCAyLjEyMiAxLjQxODIgNC41NDk2IDIuMjc5NSA2LjgxNDIgMy40MzI3IDkuMTE2MyAzLjkzNzggMTkuMjI1IDQuNDUzNiAyOS4wMjIgNC4zNDM5IDIuNjA5My0wLjAzMzUgNS4xNjMtMC43MzI1OSA3Ljc4MDgtMC42ODI5OSA2LjI4OTUtMC4yMjgzNSAxMi42ODEgMC40Njk5MyAxOC44NzItMC45NzY3OCAyLjI1NjEgMC4xNDM5MSAzLjczNDgtMS44NDM3IDUuMjY2LTMuMTczMSAwLjk2MDc0LTEuNTkwOCAzLjEzMzktMC42MzMxNSAzLjMwMTQgMC45NTEzNSAxLjM0MjQgMS4wNjUyIDIuODU3NyAxLjg5MjUgNC4yMjcyIDIuOTI3MyAzLjE5MTUgMi4yMjAzIDYuODIxOSA0LjI4MTMgMTAuODM4IDQuMTQ1NyA2LjA1NTYgMC4zNTAwNSAxMi4zMDEgMC45ODkzOSAxOC4xNzgtMC45NDA0NyAxLjQ3MjgtMC41NTMyMSAyLjk3ODktMC45ODUyMSA0LjQzODItMS41NjEzIDEuOTk0LTAuODczMDggMy40NDY4LTIuOTIxMiAzLjI4MzgtNS4xNDMzLTZlLTMgLTMuNTQ1Mi0yLjA2MDMtNi42MDcxLTQuMDQ3OC05LjM3ODItMS42OTY4LTIuOTYxMy0zLjYxNTItNS44MTg5LTYuMDQ0Ny04LjIzNTctMi40OTg3LTIuNTc4OS01LjUwNzctNC42MTc3LTguNzMzOC02LjE2OS0xLjQwNjUtMC44NjI5OC0xLjI3NzgtMi43MzA0LTAuMzUzNTYtMy44OTk3IDEuMTA0OS0xLjg4NzEgMi45MDc4LTMuNjQ1NiAyLjc3MDItNS45OTkxLTAuNzM4OS0xLjI4NTItMC40NzY0NS0yLjY1MjcgMC43MDM2NS0zLjUzMzggMS4zNjM1LTEuOTY0NiAyLjEyNTktNC42ODggMC45MzA5Ni02LjkxNDUgMC45NjA5NS0xLjI3MzQgMi40NDU1LTIuMTMxIDMuMDM0MS0zLjY4OTkgMS42MTE3LTMuMzk3OCAwLjA5NzctNy40MDI1LTIuMzE3Mi0xMC4wMDQtMS4zNzI0LTEuNTE3MS0zLjIxMTctMy4zNTcyLTIuNjExNC01LjYwMjEgMC4zOTU5My0xLjcxODQgMi40ODY4LTIuMzkxMiAyLjQwNDctNC4yNjM2IDAuNzUyOTItNC4xMTA5IDAuNDU0NTEtOC4zMiAxLjAxNzYtMTIuNDUyLTEuNTQ4My0xLjIzNzgtMy4zMjU1LTIuMjE0My00LjU3MzctMy43OTQ5LTIuMzQ5NS0yLjUxODEtNC4zNjIxLTUuNDUxMS03LjMzMjUtNy4zMDE2LTQuNTA0OC00LjUzMjQtNi4xNzYxLTExLjMwNi0xMS40MTktMTUuMTg5LTEuNTI5MS0xLjE4ODQtMy41NTQtMS44MTIzLTUuMjc4Ny0yLjI2OTctMC4zMTc3OSAwLjA0MTM2LTAuNzk3MjcgMC40ODIxOC0xLjA1MzYgMC4wOTc2NjV6bTEuOTQ1IDM2LjA3Yy0zLjUxMjctMy44MjAzLTYuOTkyOS03Ljg1NTQtMTEuNTg0LTEwLjQxOS0wLjgxOTQ2IDAuMTA0MTktMi4zMTk4LTEuMTk4NS0wLjY2OTM2LTEuMTcwNiAyLjIyNDUtMC4xNzIyMyA0LjU4Mi0wLjAzNzgyIDYuNTcyMyAxLjA2ODEgMi4xMTkzIDIuMjg1MSA1LjAyMjcgNC4yMjk4IDUuNzEwNCA3LjQ3MTUgMC4yMjA1NiAxLjAwMTggMC4yMTc4OCAyLjA1NDctMC4wMzIwOSAzLjA1MDltMjEuNzUgMTEuMTE1Yy0xLjE1MDIgMS41NzY4LTMuMjU2MSAxLjY1NjEtNC45MjAzIDIuMzU3NSAxLjQ2ODggMC44NDA4OCAzLjM1MyAwLjU3MTQgNC42MDA0LTAuNTQ0MzggMS40MzI1LTEuNDgxNyAzLjY2NS0wLjg2MDY5IDQuMTExMyAxLjE0NTYgMS4zODU1IDIuODI2MSAwLjk2OTYzIDYuMjYwNS0wLjcxNzc1IDguODYyNS0xLjk4OSAzLjMwMy01Ljc1MDggNS42NDgzLTkuNjc1MyA1LjM2LTguNzE1OSAwLjQzNTY3LTE3LjQxOS0wLjUxODcxLTI2LjA4LTEuMzc3My03LjcwODUtMS4wNjMzLTE1LjMwNi0yLjc5OTMtMjIuODYtNC42MzgzLTIuODU5OS0wLjg2NzExLTUuNzM0OC0yLjU0NjQtOC44MTYzLTEuOTQ4OC0xLjc5NjkgMC41NjU3My0yLjczMSAyLjkzNTctMS42NzM1IDQuNTM3MSAxLjc0MiAzLjUzNzkgNS42NTIgNS4wMjAzIDkuMDEwOCA2LjU4OTkgMS44MzA3IDAuNzk0MDggMy41MTM0IDEuODc4MiA1LjI0MzIgMi44NDk4IDUuMzQzNyAyLjUxNSAxMC44IDUuNDc4IDE2Ljg1OCA1LjU0NTMgNi40MTIzIDEuMDk4MiAxMi45NjIgMS4zMzI1IDE5LjQ0NyAwLjkwNjk4IDIuNzM3NC0wLjQwOTA2IDUuNDk4OS0wLjIzNjY4IDguMjQ5OC0wLjEyNzA1IDIuODM4NyAwLjA4NDM3IDUuOTIxMy0wLjIyNTc0IDguMTk0NC0yLjA5ODIgMS4xMjg3IDAuMTU0NDMgMC4yODc3NiAyLjMyODkgMC4xNTIxOCAzLjI0OTYtMC40NDkxNiAxLjM2OTYtMS4yMjQ5IDIuODIzMy0yLjU4NDggMy40NTU0LTEuOTQyMSAwLjE0NjMtMi45Nzc0LTIuMzMyNS01LjA0NjUtMi4xODI3LTMuNDE0LTAuMzQ4NjUtNi4yNzY3IDEuOTY2NS04LjY5NDQgNC4wNzY4LTIuMjIwNiAxLjk2NS00LjMwNCA0LjE0OTYtNi45MTk5IDUuNjAyNC0xLjI3OTgtMS43NTIzLTQuMDY5NS0yLjExODItNS42NjY5LTAuNTg2NjItMS45ODM4IDEuNTI0NC0yLjU0NDcgNS4wNDkzLTAuMzE3NDEgNi42MTU1IDEuNTkxOSAxLjEzNDYgMy4wMDIgMi41NjkxIDMuNzExIDQuNDI1NSAxLjMzMTcgMi45Mjk3IDEuMTk0NCA2Ljk5MjktMS41ODYyIDkuMDQ1LTIuMzQxMyAxLjY3MzUtNS4yNjc4IDIuMjQ5LTguMDYxOCAyLjY2MzMtNS43MTAzIDAuNjU3MjItMTEuNDcyIDAuMDI5LTE3LjE5NCAwLjUzNzA1LTUuMTY4NCAwLjM3NTg4LTEwLjM4MSAxLjAwNTYtMTUuNTQ3IDAuMjY1MjQtOS4wMTE1LTAuNTA2ODMtMTguMjQ4LTIuNTM0Ny0yNS43NTgtNy43Njc4LTMuOTc5NS0yLjU2OTEtNi4xMTEyLTcuMTcxLTcuMDUyNi0xMS42NzctMC43NDQ2OS0zLjU2NjEtMC40NTY3Mi03LjIzMTIgMC4wNTYyODktMTAuODA2IDAuOTAyNTQtNy42ODA4IDEuNzYzNi0xNS4zODEgMy4yNDY1LTIyLjk3NiAyLjUxNzEtMTAuOTM5IDguNTc3LTIwLjUzMyAxMy42NzMtMzAuMzkgMS40NzgzLTIuOTk1NSAzLjE2OC01LjkwMSA1LjM0NzctOC40NDY4IDMuMjI2NC0zLjk5NzEgNi45Njc4LTcuNjc3IDkuMjYxOS0xMi4zMzQgMi40LTQuMDQ5IDYuNzgzMS03LjAwMjkgMTEuNTM4LTcuMjM0NyA1LjUxNy0xLjMxNzEgMTEuNjU4IDAuMDk2ODggMTUuOTk5IDMuNzU0OCAxLjg4MyAxLjY5OTkgMy4yMjc2IDQuMjE4NyAyLjkyODggNi44MTA1LTUuODU5Mi0wLjczMjExLTEyLjA4Ni0xLjUyODMtMTcuNzA5IDAuNzg1ODgtMi4zODggMS4wMjg5LTQuNTI5NCAyLjY5Ni01Ljk3NDQgNC44NzEyIDIuMDE2NC0wLjIwOTYxIDMuNTA5Ny0xLjg0ODQgNS4wNjY3LTIuOTg0OSAyLjA3MTMtMS40OTE1IDQuNzIxNS0xLjc1NzMgNy4xNzYxLTIuMTMxNCA0LjUzNjctMC40OTYxNSA5LjE1MiAwLjA5MjU3IDEzLjUwMSAxLjQzMjYgNS4wNzYzIDIuNzgwMiA4LjQ4MjcgNy44MDQzIDEwLjY3MyAxMy4wMzUtMy4xODUtMi4xMzc4LTcuMTczMy0yLjQyNjItMTAuODk5LTIuNTA4LTMuMTQ2OCAwLjExMzczLTYuMjgyMSAwLjU2Mjk2LTkuNDM2OSAwLjM2NTU0LTQuNDEwNC0wLjA2NjMzLTkuMTI4MS0wLjI3NzM5LTEzLjEyNiAxLjg5NTctMi4zODc4IDEuNjQ1NC00LjYxMDcgMy45NTYxLTUuMjU2MyA2Ljg2MDUtMC4xNjg1MSAxLjc5MjkgMS43NTcxIDIuNDY2NCAzLjIyNjQgMi4zNzEzIDEuODQ4NSAwLjA2OTU4IDMuMzM1MSAxLjI4NDcgNC45NjYzIDIuMDA4MyA1LjA1MTEgMi43MDY1IDkuOTg1MyA1Ljk4OTQgMTUuNzM3IDYuOTcyNCA1LjE0NzIgMS41MTEgMTAuODMzIDEuMTY3MSAxNS43MzItMS4wMzU4IDEuNjQ0IDEuMjU4OS0wLjQ0OTMyIDMuMDgyNC0xLjg1MyAzLjMwMDEtMi4yOTg2IDAuODkyMDQtNC44NzAyIDEuMTk2NC03LjI2MDkgMC40ODc0MS00LjczMTUtMC42OTctOS42NjYtMC4xMjQwNS0xNC4yNTUtMS43MTEzLTIuMjExLTAuNzQ1MjEtNC4yMjY0LTIuMTAwOC01LjU5NTctNC4wMDc2LTAuNDUyNDYtMC45MjYwMS0zLjQyMTQtMi4wNTQ1LTIuNTQ4OS0wLjEzODUxIDEuNDcxMSAwLjc5NTM4IDIuNDk2NiAyLjE2MTIgMy43NDcyIDMuMjU1MSAzLjc1NTEgMi44Mjc2IDguNjI0MSAzLjA2NTYgMTMuMTI4IDMuNDIzMSAyLjU0NzQgMC4yOTgyNCA1LjEwODIgMC40NTIzOCA3LjY3MDIgMC40MzE0NiAxLjM5NC0wLjM5NTAxIDIuOTg4NCAxLjEwOTEgMS4zMTI4IDIuMDgxMy0yLjU2MTYgMi40NjE5LTUuNzg0NyA0LjEwODctOS4xNzEyIDUuMTExNi0xLjcyMjEgMC41NDE1My0zLjQ3MzQgMC45ODI0Ny01LjIxNTEgMS40NTUxIDEuNTUxNCAxLjM4MDUgMy43MzMgMC40NzIxMiA1LjM5ODUtMC4xNzQwMSAyLjY2MjctMS4wODk3IDUuMDI4Ny0yLjgwMjMgNy43NDUtMy43NzI3IDEuNzg0NS0zLjU3MzggNi4zMTA2LTQuMzgyIDguMzkzOS03LjY5MSAxLjAwNTgtMS41ODM4IDAuMDI0MTQtMy4yNjY4LTAuOTg1MTgtNC41MjI5LTEuOTM1OS0zLjIzMDgtMy4zODQtNy4xMDg2LTYuODE1MS05LjA4MDUtMS4yNTM0LTAuODc0NzQtMi45NDktMC42ODk3NS0zLjk5OTktMS44NTQ2LTQuMjQyMy0zLjA4NDktOS44MzAyLTMuNzY2NS0xNC44ODctMi43NzE0LTIuMDI0IDAuMTIyMDgtNC4wNjExIDAuNDQzLTUuODY2OCAxLjQxNTgtMi43ODI0IDEuMzc1Ni01LjA3MzEgMy41MDg2LTcuNTY4MiA1LjMxNTggMC42MjYwOC0zLjk3NzEgNC4xOTM2LTYuODA3NSA3LjkxNDUtNy43OTk4IDIuOTU2NS0wLjg5MTYxIDYuMTA1NC0xLjAyMjMgOS4xNTY0LTAuNjI2MzUgNS44MTM5LTAuNzUyOSAxMi4yNjQgMC45MDExNCAxNi4xNzEgNS40Nzk2IDMuNTk2MSAxLjg0NDggNS4yNjU4IDUuNzI3OSA3LjUwODEgOC44NjEzIDAuNDcxMjMgMS4wODQ1IDAuOTU3NTQgMC41MjggMC42NzA4NS0wLjM0OTgxIDAuMDgxMDgtMy4wMjUyIDEuMDc0OS02LjM1OTktMC42NTU2MS05LjEyMS0xLjA3MTYtMS4yODg4LTIuNTE4OS0yLjQ3MjYtMi4wNDM2LTQuMzMyLTAuNzI5NDctMy40NDQ1LTMuNTg1Ni01LjgyLTUuMTIyNy04Ljg2MzQtMC44MDc4Ni0xLjg3NyAxLjU4MTMtMS45MzYxIDIuNzc1MS0yLjI3MjEgOS4zNzM4LTIuMTU4NiAyMCAxLjY1OTEgMjUuNDQzIDkuNjg2My0zLjczMjcgMC42MTkwNC03LjcyODYtMC41NDUzOS0xMS4yODMgMS4wNzc0LTAuODg0NzQgMC4zNzQ2MS0xLjU1NTQgMi40MDIyLTAuMTAxMTYgMS4yNDg0IDMuNzExOC0xLjcyNjYgOC4wNDg4LTEuNTYwNyAxMS45MjYtMC41MTcwNiAxLjk1MDEgMC41NTg0NSAzLjk2NTEgMS40Nzk3IDUuMDg4OSAzLjI1NDEtNi4wNDYtMy4zMjgyLTEzLjk5OC0zLjY0NC0xOS44OTQgMC4xODQ3Ni0wLjk2ODYxIDAuNjA1OTEtMS45MTY2IDEuMzY0Ny0yLjM5MDMgMi40MzQ3IDEuNDE3NSAxLjA3ODMgMi41NjYxLTEuMTM3IDMuNTkxLTEuODQyOCAzLjcyNy0yLjMzMzQgOC4zODk1LTIuNDM1NiAxMi42MDUtMS42ODk0IDEuNDkzNyAwLjMyNDU3IDMuMDY5MSAwLjU1ODU0IDQuNDQ1NCAxLjIxNTYgMS4xNDMyIDAuNzcyODggMi4wNTk2IDMuODgwNyAwLjI5NzAxIDQuMDcwNC0wLjk2MjM4LTEuMTk5OC0xLjQxMTUtMi44NjMxLTMuMTc3NS0zLjIwNjUtNC4zNDI3LTEuOTg5NC05LjczNTktMS4zMjQ0LTEzLjUyOCAxLjU1NTktMi4xNTU5IDIuNTUzMy01LjA5MTIgNC41NTYzLTYuNDQ5OSA3LjcwNDctMC41OTM2OCAxLjYyNDEgMC40OTE4MiAzLjc4NzEgMi4zMzkxIDMuNzkgNS40MjMyIDEuMDA2OCAxMS4wOTQgMS4zODI1IDE2LjUwMiAwLjA5MTE4IDEuNjYyNi0wLjI4MTYxIDIuNzc4Mi0xLjU3NDcgMy44OTY0LTIuNzE0MiAwLjUyMDgtMC41NTU3IDEuNTYzOC0wLjgzODQgMC45NTA4NSAwLjIyNzAyLTAuMzQwNTkgMS43MDY5LTAuOTkxNiAzLjY1OTQtMi44MDIyIDQuMjkwNS0xLjY3MjcgMC43MjQxOC0zLjYyNjYgMC42NzY3OC01LjE3MDggMS43MTY4IDAuMzAzNjIgMS4wNTU3IDIuMzI5NiAwLjMyMTYyIDMuMzA2NiAwLjQ5MzA1IDAuMTczOTctMC4wMTU1OCAwLjM0NzYyLTAuMDM0NzMgMC41MjA3OC0wLjA1NzYyem0tMTcuNzQ2IDM5Ljk4OWMyLjYyNzYtMy4zNDM1IDYuMzY2LTYuMjY1MyAxMC43NjItNi41MDY4IDIuNzM5OC0wLjA1OTQ1IDUuMzExMiAxLjc2NDYgNi4zNzgzIDQuMjQ2NiAwLjkyMjA0IDEuMzg1NSAyLjY5ODggMS41OTQ4IDMuOTUyMiAyLjU2ODkgNi4xMjUxIDMuOTA0NiAxMC4yNzMgMTAuMTM5IDEzLjUyIDE2LjQ5NSAwLjgyOTY1IDEuNTQwOCAyLjEyNTEgMy4xMTE5IDEuNzEgNC45ODYxLTAuMzMyMzYgMi4yNjgyLTIuNTIyNSAzLjQ4NDctNC4zODkyIDQuMzk4Mi0yLjcxNDMgMS4yMTk3LTUuNjc3IDEuODA4NC04LjYxNDQgMi4xNjM4LTMuNjgyNCAwLjA0NzMtNy4zNzI5IDAuMDg2OC0xMS4wNTEtMC4xMzc4LTIuNzU0My0wLjIzNDM3LTUuNTg4NC0wLjYxNzMzLTguMDk1MS0xLjgzNDEtMy4xMzQ2LTEuOTI2MS02Ljc4MTItMy42NDMyLTguNTI0Ni03LjA4MjItMC43Nzc5Mi0xLjY1OS0wLjc0Njg1LTMuNTg1MS0wLjMwNDk0LTUuMzMxNy0wLjEyNzM4LTIuOTI2OC0yLjc0NDgtNC43MjQyLTQuNDM3Mi02LjgwNzMtMS4zMDUzLTEuMzIyNi0yLjAxNzYtMy44MTQ2LTAuMzM3NDktNS4xMzg2IDEuNDY5MS0xLjI2NzIgMy42MjA1LTAuNDMwNSA0LjMyOTcgMS4yMjcgMS4wMTcgMS42MDk0IDEuODI3IDMuNzU4MSAzLjg2NzMgNC4yNzk1IDIuMTYwOS0wLjM1MTE3IDEuMTI3IDIuNDExIDIuMDc0OSAzLjYwMSAxLjcyNjcgNC4yNDQ4IDUuNjI4OSA3LjU1NTYgMTAuMDczIDguNjIxNCAxLjU4Ni0wLjA5MyAxLjk2OTkgMS40NTUxIDIuOTA3MSAyLjMwMTMgMi41Nzc2IDEuNzk5IDYuMjE4MyAwLjQwMDA0IDguNjU3NiAyLjU0NTYgMS43MzMxIDEuMDkyOCAzLjc2NzMtMC44MTgwNyA1LjEyMzMtMS44ODg3IDEuMzE4Ny0xLjIxODMgMi42Mzg0LTIuNjMwMiAzLjAxOTEtNC40NDQ4LTAuNDU3OTEtMS41NzYyLTEuMzg5OCAxLjQyNTgtMi4yMDI1IDEuNjkxLTEuNTQ5OCAxLjU2ODMtMy4yMjY2IDMuMjY2NS01LjQ1OTQgMy43NTM1LTEuOTA1OC0wLjEwMjY1LTIuMDQ1OC0zLjE0MTItMC40MDk0Ni0zLjg0NDMgMS4zOTY4LTEuMjM3NCAyLjU0MTItMi43NDIgMy40NTYtNC4zNjM5LTAuMjU3MjUtMS4yODUtMi4wOTE1IDAuOTc5MTItMi40NDc0IDEuNjkxOC0xLjUzMDYgMi4zNTc2LTQuMDE3OSA0LjE5OS02LjgwNSA0LjY5NzgtMS44MzM4IDAuMjMyODItMy4xMTcyLTEuNjY1MS0yLjgxNDItMy4zNDY0IDAuNDI3MDgtMi41MTE1IDIuODI4My0zLjg1ODUgNC4zODUxLTUuNjQwMiAwLjg0MDE5LTAuNDgxNjcgMS4zNzQtMi42MDgyLTAuMTczMjgtMS44NTU1LTMuMDAzNyAxLjYyMjYtMy4zNzM5IDUuNjExMi02LjE0MDggNy40ODUyLTEuOTg3OSAwLjg5MDA5LTMuOTExNi0wLjgyMzU5LTUuMjEzNi0yLjE1MjItMS4zODgxLTEuNjIwMy0wLjY2MTc5LTMuOTI4OSAwLjc3NzA1LTUuMjM4OCAxLjkyMzctMi4xODU3IDQuNjc0NC0zLjI4MDggNy4xNjMxLTQuNjUxOCAxLjQ4Ni0wLjkzNzc4IDIuNzgzMS0yLjE3MjUgMy43OTg0LTMuNjA2Mi0wLjk0NDA2LTAuNzk2NTgtMi4yOTYyIDEuMDAwNC0zLjEzOCAxLjU1NzctMC42MzYyNCAwLjgwNDE5LTEuNDI4OCAwLjM3NTk4LTAuNjk2NjMtMC41MzQwOSAwLjkyMDgyLTIuMDA2MS0xLjUyNzgtMy4wNDAzLTMuMTM1Ny0yLjk3NjEtMi45NzQxLTAuMjgyMzgtNS4yNDc3IDIuMDA3Mi03LjUyMDYgMy41NTMzLTEuNjgxNCAxLjI0NDMtMy45MDI4IDIuMTgwNi01Ljk2OTUgMS4zMjMxLTEuOTg2OS0wLjUyOTA3LTIuNTU3MS0zLjgzOTEtMC4yNjM5LTQuMzUzMSAxLjY2MTktMC43OTk4OCA0LjcxODMgMS4xMzk1IDIuNjI2NyAyLjc0ODgtMS40MTA1IDEuNTY3NiAxLjkyOSAxLjExNDEgMS43MzI1LTAuMzc0OTIgMC41MDk5Ni0xLjYyNzMtMC4yNjM1My0zLjcxNjMtMi4xNzAzLTMuODI3N3ptMjMuODQ0LTMyLjkyYzEuNTY0MSAxLjY2MzkgMS42NTY3IDQuMzk4MiAwLjI3NDIyIDYuMjAyNC0xLjg1MjMgMi44ODE5LTQuODQzNCA0LjcxMDYtNy4xNDQ4IDcuMTgzNy01Ljk0NDQgMi4yNTkyLTEyLjM1OSAwLjcwNDQ3LTE4LjUxOSAxLjE2NTItNy43MzgtMC4yMzQzOC0xNS4yMjktMi43ODk5LTIyLjk5NC0yLjY4NDktNS41OTIyLTEuMTEzMy0xMS4yLTIuNTU2LTE2LjI5LTUuMTczMy0wLjg2NTc5LTAuOTQyNDItMi45MjMtMS40NDM2LTEuNTU4MiAwLjI5NzEyIDEuMzk2MyAxLjc2NzMgMy43NTE2IDIuMjg3MiA1Ljc4NjUgMi45NTgzIDIuNjAzOCAwLjczMDU2IDUuMzExNCAxLjIxNjUgNy43MzcgMi40NjM5IDYuMzQ3OSAxLjQyNjMgMTMuMDEzIDAuODAyNDQgMTkuMjY5IDIuNzIzNiA4LjMwMTUgMS4wNDU1IDE2LjcwNiAwLjk3OTQ1IDI1LjAzOCAwLjM2MjE2IDIuMTM2OC0wLjY2OTk2IDMuNDg5Ny0yLjY2NDIgNS41NDM2LTMuNDkyMiAxLjg3NTIgMC43MjIyNyAwLjQ1MzUxIDMuMjYxMS0wLjM1MDkgNC40MDQyLTIuNzM0MSAzLjk2MjItNy45NzI2IDUuNjM4NS0xMi42MDQgNC43NTg5LTEuODgzLTAuMjk1MzQtMy43NjIgMC4yMjY3MS01LjY1MjYgMC4yMDgzMi02LjIxNzggMC4yNzU4OS0xMi40NzggMC4xNDE4Mi0xOC42NS0wLjY4ODQyLTUuMjYyOC0wLjgzODkxLTEwLjY3My0xLjg2NjctMTUuMjM5LTQuNzczNS01LjE4OTktMi42NTE4LTEwLjY1LTQuOTc5NS0xNS4xOTYtOC42OTE1LTEuMTY1LTEuMDMwOS0xLjA0OTgtMy4xMDE2IDAuNTA1MDgtMy42ODE1IDEuNjMxMi0wLjg0MDMyIDMuNzI0Ni0wLjQ1NDg3IDUuMDk1MiAwLjcxNDU1IDcuOTM2NCAyLjczNzMgMTYuMjczIDMuOTcxNiAyNC41IDUuNDg2MSA4Ljk1NjcgMS4xNDI0IDE3Ljk4NyAxLjgyNjMgMjcuMDE5IDEuNzY5OCAzLjk5MDctMC4yMDQ1NCA4LjMyNjEtMS41MzgyIDEwLjY5Ni00Ljk3NDcgMS4yODg2LTEuOTk1NSAyLjE5NDUtNC4yMjg0IDIuNzM1NS02LjUzODN6bS0xNS4xMjItMzkuMDRjLTYuMzI1Mi0zLjQ4My0xNC4wNTUtMy40MjMtMjAuODE3LTEuMzIwOS0xLjY5MzEgMC44NjY5Ny0zLjE2NzItMC42Nzg5OC00LjM3NDktMS42NjQyLTEuODQ3OC0xLjc3NDQtMy40Mzc2LTQuNTkyMi0yLjI1OTQtNy4xNTIyIDAuOTI2NDEtMS43NjY0IDMuMDE1OS0yLjI1NDQgNC42MDc1LTMuMjE1IDUuMjU5LTIuNDEwNSAxMS43MjMtMC45MDM5IDE2LjAwNCAyLjc4NDIgMS45MTY4IDEuODU1IDMuMjEwMyA0LjIzMTkgNC42NjU3IDYuNDQwNiAwLjgwNDg4IDEuMzMxMyAxLjU0NTQgMi43MDM1IDIuMTczNCA0LjEyNzV6bS0yMy4yNjcgMTYuNjA3Yy03LjAzMzYgMS4zMjkyLTE0LjI2Ni0wLjQ2MjE1LTIxLjI4MSAxLjA4NjUtMS44MDQ4IDAuMzM2MDYtMy42MzQzIDAuOTM3MjYtNS40ODcgMC42MTEwNi0wLjg1OTQyLTEuNDQ3OSAxLjIwNTItMi41NjEgMi4xNzYtMy4zMTI5IDQuNzA1Ni0yLjg1NiAxMC40MTktMi45MDYzIDE1Ljc1LTIuNzQ3IDMuNDE5NCAwLjI5OTkgNS43NjA2IDMuMjA1MiA4Ljg0MjQgNC4zNjIzem0tMy43MjYgMTEuMjg1YzIuMDEzLTEuOTU4IDEuNjQ3OC01LjAyMDkgMS4yODk4LTcuNTQzOC0wLjczNTI0LTEuNDk4OSAwLjM2NTk4LTIuOTEzIDEuOTk3OS0yLjU3ODggMy4xMzA1LTAuMjQ5OCA1Ljk1MTUgMi4wMDI1IDcuMDg2NiA0LjgxMjUgMC41MzA0IDEuMTkwNyAwLjgzNjY4IDIuNDc2NiAwLjkzNjAzIDMuNzc0OS0zLjU3NDggMS4zMjY1LTcuNDkzIDIuNjM5My0xMS4zMSAxLjUzNTJ6bTIzLjk2Ny0xMC42NDZjMy42OTU0LTIuNjE5IDkuMDE4Mi0zLjIzMzYgMTIuOTQ5LTAuNzgyMjggMS40MDEgMC45NzUyMiAyLjM2MDQgMi40NjQ5IDIuOTgxNCA0LjAzMi0xLjI2NzcgMC44OTc5OS0yLjUxNjUtMC45MjQwMy0zLjc1NzctMS4yMzAyLTEuNTk1My0wLjkwNjE1LTMuMzM3My0xLjgwNTktNS4yMjk3LTEuNjI2LTIuMzA4Ni0wLjIyMDI3LTQuNjI5LTAuMjU3ODItNi45NDI5LTAuMzkzNTZ6bTguMDczIDEuNzE5YzIuNDQ1MiAwLjg5NTUgNS4yMDg2IDEuNTA3MyA3LjAyMDEgMy41MTc5IDAuOTc4ODEgMS43OTExLTEuMjYxMiAzLjA5MDYtMi42MTE1IDMuNzUzMy0xLjAyNTUgMC44NDA2My00LjM3MjYgMS45MTM5LTMuNjI4Ny0wLjQ4NTMzIDEuMTk0MS0xLjUyMjUgMS4wMjc1LTMuNTU0OSAwLjE2MDQ2LTUuMTkzOS0wLjI2NTI3LTAuNTU3MjktMC41ODU2OC0xLjA4NzItMC45NDAzMi0xLjU5MTl6bS0xMS44MTcgNjQuMjZjLTEuMTAzNS0xLjczOTYtMC40Mzc1Ni00LjE2MTQgMS4zMjU2LTUuMTg1NCAyLjIzMjktMS43MDA3IDUuMDQzLTIuMzkyMiA3LjMwOTQtNC4wMzg2LTAuODk1MzcgMi45MjA2LTMuNTczMiA0LjcxMDktNS43MjEgNi42NzU4LTAuOTg2NTEgMC44MzE3LTEuOTc3IDEuNjYwMy0yLjkxNCAyLjU0ODJ6bTIuNDk0IDMuMDM2Yy0xLjcyMjgtMC45ODQ2OS0xLjE0LTMuNDE2NiAwLjM2My00LjMwODQgMi4wMzIyLTEuODgwMSA0LjYxNi0zLjE2NDcgNi4yNzg3LTUuNDMzNy0wLjIyMTQzLTEuNDA4NiAyLjk3MjYtMS45NjEgMS42ODYzLTAuMTg3ODctMS4xNjUzIDEuOTY0LTIuNzc0OCAzLjY0NzgtNC42MjE0IDQuOTgzNS0yLjA1NjUgMC43MzEwNy0zLjAyODQgMy4wNDkyLTMuNzA2NSA0Ljk0NjV6bS00My41MDgtNjYuODU1YzIuNTE0OC0wLjEzOTM0IDQuOTQxNS0xLjE3MTggNy40ODM5LTAuOTE3NTEgMS4zMDk2IDEuMDc2Ni0wLjgyMDU5IDMuMDU1My0xLjc4MjQgMy4zNzExLTIuMDkxNS0wLjMzNzI3LTMuNTkzNS0yLjE2Ni01LjcwMTUtMi40NTM2em0xNy40NDQgNC44OWMtMi4wODcyLTAuMDA1OC0yLjMwODEtMi44NzE4LTAuMjMxMTUtMy4xNTYyIDEuOTMyNi0wLjk2NDAyIDMuMjgxNSAxLjQyOTYgMS42OTk0IDIuNzAxNS0wLjQwMTk2IDAuMzQ3NTYtMC45NTE5MyAwLjQ1MTg1LTEuNDY4MyAwLjQ1NDc1em0yNC4zMzgtMy44NzdjLTEuMjU2NyAxLjg2MDEtMC41NjgyOCA0LjIzODMtMS4zMTcgNi4yNTgzLTEuNzc0MyAwLjk1MTE5LTMuMDU4NS0xLjkxMDUtMS41MTkzLTMuMDMwMSAwLjc2MjctMS4yMjQ1IDEuODQxOS0yLjE5OSAyLjgzNjQtMy4yMjgxem0xMC4zMjIgMi42OGMtMC4wNTQ5OSAyLjAzMjYtMi44NjA4IDEuOTM1Ny0yLjY5MTgtMC4xNTk4Mi0wLjA4Njk5MS0xLjkxOTggMi43Mjc1LTEuNDA1MSAyLjY5MTggMC4xNTk4MnptLTQwLjk1NS0yLjA3NGMtMS44MTA1LTAuNDkzNTUtMC4wMjUxNC0yLjcxNCAwLjk1MzIxLTEuMjAwNiAwLjIxNjI5IDAuNTkzOTUtMC41NDY3NSAwLjk1OTE2LTAuOTUzMjEgMS4yMDA2em0zMy43MjYgMS4wNzhjLTEuNDgzOC0wLjM5NjIxLTAuNjcwNjItMi42NDQ3IDAuNjk1NzUtMS41NzYzIDAuNjU0MSAwLjUxMTIyLTAuMTkxMTYgMS4zNTQ2LTAuNjk1NzUgMS41NzYzem0tMzMuMDg0IDIuMTg3Yy0xLjg4MjctMC4yNDkwNi0wLjE0NTkzLTIuNzI1IDAuODkzODQtMS4zMjE1IDAuMDU1MjkgMC41NzA0Mi0wLjUxNSAwLjk4NjY2LTAuODkzODQgMS4zMjE1em0zMy4zMjMgMS4zMTZjLTEuOTA3OC0wLjM5OTE2IDAuMzc0NjUtMi41NzQ1IDAuNzQyODEtMC44MTAzNi0wLjEyNTYxIDAuMzU3MzItMC40NDQwMSAwLjYwMTMyLTAuNzQyODEgMC44MTAzNnoiIGZpbGw9IiMwYTEzMGEiLz48cGF0aCBkPSJtOTkuNzYxIDY1LjE2NGMtMC43Nzg4MSAzLjQ2NTMtMi40NTk1IDYuODQxMi01LjI3MzggOS4wOTI2LTAuOTQ3MzkgMC44Mjg0Ny0yLjA4OSAxLjQyMTMtMy4zMzIxIDEuNjQ2NS0xLjAyNjQgMC4xNzY2Mi0yLjAyOTIgMC40ODIyOS0zLjA2MjMgMC42MTI5Mi0xLjU3NzIgMC4wMTA3OS0zLjE1MzggMC4wNTA2MS00LjcyNzIgMC4xNzM4OS0xLjYwMTcgMC4xMjUxNC0zLjIwODggMC4wNjQ2Ny00LjgwMzYtMC4xMTUyMy0yLjcyMzctMC4xOTc1My01LjQ1NTctMC4zMTE1OS04LjE2NzYtMC42NDE0LTEuODcwMiAwLjAzNjQ0LTMuNzQyLTAuMDUyNzUtNS41OTUzLTAuMzE2MzEtMi4xODU2LTAuMzIwMjctNC4zODEtMC41NjMzNC02LjU2OTMtMC44NjExMi0zLjIyMTQtMC41MTU3NC02LjQzNjUtMS4wOTQ5LTkuNjAxNi0xLjg5MTctMi45MTAxLTAuNzIwMzctNS45MjkzLTAuOTI3MDEtOC43OTk0LTEuODIzNy0zLjIxODItMS4wOTA4LTYuNDU1My0yLjE3OTktOS44MDgyLTIuNzgwNi0xLjE0OC0wLjExODg5LTEuNjQ5MiAxLjI1OTQtMS42NTIgMi4xOTc2LTAuMDIxNzUgMC45OTA2MyAwLjUyMTE5IDEuOTI4MSAxLjM3MDYgMi40MjgxIDIuMTEyNSAxLjYzNTcgNC4zNDA1IDMuMTk4MyA2Ljg5MDMgNC4wNjk3IDAuOTM3MTMgMC41MDg3OCAxLjc5ODEgMS4xNTQ5IDIuNzQ2NSAxLjY0OTggMy4zMDQ2IDEuODM2NCA2Ljg2MjEgMy4xNjQgMTAuMTc2IDQuOTgzMyAyLjE4MjEgMS4wMjMgNC42MDE5IDEuMzUzIDYuOTY1OCAxLjY3OTggMS40NTk5IDAuMTEyMjYgMi45NjQ5IDAuMTMyNjggNC4zMjggMC43MzYxMyAyLjYwMTEgMC44MTM0OSA1LjM0NTcgMC4xOTE5NyA4LjAwNzQgMC41MDczNiAxLjk2MjcgMC4xOTg3MiAzLjkzNTUgMC4yODk1NyA1LjkwNjcgMC4xNzUzNCAzLjAyNjMtMC4wODYxOCA2LjA0NDktMC4zMzE4IDkuMDYzMy0wLjU0NDg5IDIuMDE1Ni0wLjA2NjQ5IDQuMDI2MSAwLjU5NjA0IDYuMDMwNCAwLjEzNTM4IDIuNDI1My0wLjQ2NzM2IDQuNTk2Ni0yLjAwNDQgNS44OTc1LTQuMDk3NCAwLjczNjI0LTEuMTIxMSAyLjAyMDMtMS45ODI2IDIuMTEyNi0zLjQzMzMgMC4wNTE2NS0wLjYxNDAyLTAuMTQ1LTIuMTgxNi0wLjk5ODQtMS44Nzg0LTEuMTM5NSAwLjkyNDY5LTIuMzA5NSAxLjgyMTktMy4zNTQxIDIuODU3OC0xLjEyNSAwLjk0NTE0LTIuNjUwMSAxLjAwMzItNC4wNDUyIDEuMDc3LTIuNzQ5MyAwLjE3NjI2LTUuNTA1MSAwLjM4Njg3LTguMjYxMyAwLjMxMzUyLTEuNjE1MS0wLjExODc3LTMuMjM1Ny0wLjI3NjU1LTQuODUzOS0wLjEwNzE4LTIuMDc1OSAwLjA0NTk3LTQuMTUxNS0wLjExOTI3LTYuMjIxOS0wLjI2MTEtMy4wMDg5LTAuMjI4OTgtNi4wMDA2LTAuNzA5MDktOC45MDk0LTEuNTE4Ni0yLjUwOTUtMC42MDE1Mi01LjEwMzktMC43MzY3Mi03LjY3NS0wLjY0NDI2LTEuOTY2Ny0wLjA4MTEzMy0zLjkxODMtMC40MTk3LTUuODM4LTAuODQwNjgtMy42MTk1LTAuODY3NzUtNi45NjE4LTIuNjEyOC0xMC41OTYtMy40MjY2LTEuNjgzMi0wLjMxNTgtMy4wMzk2LTEuNDM1Ny00LjMyNjUtMi40ODg4LTAuNzE2NTYtMC40MDQzOC0wLjM0MjAyLTEuNjkwMyAwLjUwMS0xLjEwNTYgMS4wNzMgMC41NzQ4MSAxLjg3ODUgMS41NTQ4IDMuMDMgMS45OTkyIDEuMzI0NSAwLjU5OTMzIDIuODEwOSAwLjY2MDA5IDQuMTUyNCAxLjIxNjYgMi44NjQyIDEuMDE3NyA1LjcwNDcgMi4xNjUxIDguNjkxMiAyLjc4MTQgMi4xNjY5IDAuMzk4MSA0LjM3MTMgMC42MDQ1NSA2LjU3NTEgMC41MzQ2MyAyLjc0NTkgMC4wMzA5NiA1LjQ0NTQgMC42MDQxNCA4LjA5MTggMS4yOTI0IDIuMDU4IDAuNDgxMzcgNC4xNjU4IDAuNjgyMjUgNi4yNyAwLjgyMzc0IDEuODA4NiAwLjIxOTM4IDMuNjI4NCAwLjQ2NzA3IDUuNDUzMiAwLjI5NjA4IDEuODk1Ni0wLjEzMTIyIDMuNzk2OC0wLjA4MTIyIDUuNjg0OCAwLjEyNzc0IDIuNTExOSAwLjI3MjU3IDUuMDEyLTAuMjQwODkgNy41MjIyLTAuMjc5MDMgMS4zNDYyLTAuMDUyNzkgMi44MDgyLTAuMDEzNzggMy45NTU1LTAuODQwNzggMC43ODctMC41MzIxIDEuMzE1Ni0xLjM2MDMgMi4xMzI2LTEuODYwNyAwLjgzOTg1LTAuNTY1NDkgMS41OTcxLTEuMjQ1NCAyLjE4MjEtMi4wNzU1IDEuMjQ4Ny0xLjU0IDIuNjg4Ni0yLjkyNTMgMy44MzQxLTQuNTQ3NCAwLjYxNTY1LTAuOTI5MyAwLjYzNjg2LTIuMTk0OC01LjVlLTQgLTMuMTItMC4zOTY0OC0wLjYxODctMC41ODg2Ny0xLjM2MjctMS4wOTgyLTEuOTA4MnoiIGZpbGw9IiM5ODNiMWYiLz48ZyBmaWxsPSIjZmZmIj48cGF0aCBkPSJtNTcuNjQ3IDU0LjAxNmMyLjk5MDMgMC41MzIwNiA2LjIwNjEgMC41NDc4NiA4Ljk5OC0wLjc5MjkgMC42ODY0My0wLjUyMDU3IDEuNjI0OC0wLjM1MTE0IDIuMzE2Ny0wLjc0NzgyIDAuMTU5MzMtMC45NTAyMS0wLjU4OTI5LTEuODQ2MS0wLjgyMjY0LTIuNzYwMy0wLjUxMzQyLTEuMTgzOS0wLjkwMzYxLTIuNDQtMS41OTI4LTMuNTM2OC0wLjUzODMzLTAuNzI1OTEtMS4yNDcxLTEuMzEwNS0xLjk2MzMtMS44NTAzLTAuOTAzODUtMC4yNTc1OC0xLjg3MjQtMC4xNDI1Ni0yLjgwNDEtMC4xMzY3Ni0xLjIzNzYgMC4wNTQxNy0yLjQ3MDcgMC4xODI5NC0zLjcwODkgMC4yMjU4IDEuMTYwNiAxLjU4NCAxLjYxNTkgMy42NTM1IDEuMTc2IDUuNTcyNC0wLjI5NTY3IDEuNDIyOC0wLjkxNTAzIDIuNzUzNC0xLjU5OSA0LjAyNjd6Ii8+PHBhdGggZD0ibTg5LjY4NyA0NS4wODhjMS4yMTYyIDEuNjk2OCAyLjA2MjQgMy44NzU1IDEuNDU4NyA1Ljk2NjYtMC4yNDUxNyAwLjkwMDMtMC43MDY0NyAxLjcyODgtMS4yNzE3IDIuNDY2NCAyLjIwODItMC4wMTk1NiA0LjQ0MzktMC44ODExOCA1LjkxMzktMi41NjQ3IDAuNDcxMDQtMC44MDMzOSAxLjU0MTktMS4zMDc5IDEuNjg5Mi0yLjI0ODItMC41MzU1Ni0wLjY5NzMtMS41ODMxLTAuNzc1MDctMi4zMjM1LTEuMjIwOC0xLjE5MjgtMC41MDk3NS0yLjMzNC0xLjEyOTktMy40MjQ3LTEuODI5NC0wLjYzMjEtMC4zMjk5Ni0xLjMzNjUtMC40OTU4MS0yLjA0Mi0wLjU2OTk2eiIvPjxwYXRoIGQ9Im0zNi44NTYgNDUuNTNjMi4wMDk0IDEuMTUwOCA0LjU2ODcgMS4zMjE5IDYuMjQgMy4wNDggMC4yMDcwMi0xLjYyMDMgMS4yOTExLTIuOTI3OCAyLjM1Ny00LjA5LTIuODgwMiAwLjIwNTYxLTUuNzYxIDAuNDg0MS04LjU5NyAxLjA0MnoiLz48L2c+PHBhdGggZD0ibTU0LjMgNTAuNDJjMC45Mjc2OSAwLjE2MjY0IDEuODA3Ni0wLjQ3NDg0IDIuMTA5Ny0xLjMzMzMgMC4zNjMxLTAuNzIwNyAwLjA4MzIyLTEuNjg3LTAuNjgwMTMtMi4wMTE3LTEuMDMwMy0wLjU5MDYxLTIuMjkwNyAwLjAwOTctMyAwLjgyNTE2LTAuNTU4NDYgMC43MzQxOC0wLjMzNzY2IDEuOTU2NSAwLjU1OTk4IDIuMzEyMyAwLjMxNDE4IDAuMTQ4MzggMC42NjQ4MyAwLjIwNjY4IDEuMDEwNCAwLjIwNzU4eiIgZmlsbD0iI2ZlZmVmZSIvPjxwYXRoIGQ9Im03OC42MzggNDYuNTQzYy0xLjM0ODEgMS4zNzMyLTIuODU2MSAyLjczODctMy41MDM1IDQuNjA0My0wLjM2NDE0IDAuNjk0MjgtMC4xMTg2MiAxLjU5NzcgMC43NzY2NCAxLjU5MjggMC42MDk2NiAwLjA3NTgyIDEuMzA4MiAwLjUxNDIyIDEuODY0IDAuMzUxODUtMC4zOTI2Ny0xLjMwMjQtMC4zNzY2Ny0yLjcwNzUgNS42OWUtNCAtNC4wMTQ3IDAuMjI3NTItMC44NjM5MiAwLjU0Njc4LTEuNjk5NiAwLjg2MjMxLTIuNTM0MnoiIGZpbGw9IiNmZmYiLz48cGF0aCBkPSJtODguOTYgNDkuMjIyYzAuMTQzNDctMS4wNDkyLTAuOTQ2NTUtMS41NTI1LTEuODM4Mi0xLjQ2NC0wLjk4OTE4IDAuMDYxNC0xLjI1OTcgMS4zNTg1LTEuMDQ5NSAyLjE1NzMgMC4yMTI3NiAwLjk2MzkgMS40NzM5IDEuMTI2OSAyLjI0MjIgMC43NzU4OSAwLjUyMzMxLTAuMjgyODYgMC42NTYzNS0wLjkyMjg5IDAuNjQ1NTctMS40Njkyem0tNDAuOTU1LTIuMDcyYzAuODA2OTEgMC4xMzE5NyAxLjMyMDUtMC45MjU1MiAwLjgwNDA1LTEuNTA3NC0wLjYyODIyLTAuNzA5OTUtMS44MzQxLTAuMDk5MTYtMS43MDQyIDAuODEyODggMC4wNjQ3OCAwLjQyOTMyIDAuNDg0MTggMC43MDU0MSAwLjkwMDE0IDAuNjk0NTZ6bTMzLjcyNSAxLjA3N2MwLjg0NjE3LTAuMTc0MzggMS4yMzY0LTEuNDY4NyAwLjMyMDExLTEuODIxMi0wLjg1Mjk5LTAuMzMxNDYtMS40NTM4IDAuODY2MzUtMC45NDA0NiAxLjQ4MjQgMC4xNjI5OCAwLjE3NjIxIDAuMzk1OCAwLjI2OTM0IDAuNjIwMzUgMC4zMzg4em0tMzMuMDgzIDIuMTg3YzAuODI5MyAwLjAwNDQgMS4wOTI5LTEuMjk2MyAwLjQxOTk4LTEuNjgwNC0wLjcwNTE4LTAuMjU1NjgtMS41ODQ4IDAuMzkxOTQtMS4yODc0IDEuMTY1NyAwLjExMTQyIDAuMzcwMjUgMC41MTc5NyAwLjUwNTQ1IDAuODY3NDQgMC41MTQ3em0zMy4zMjMgMS4zMTZjMC43NDU2MSAwLjA3MDYyIDAuOTU4NzItMS4xNDI4IDAuMzUwMjItMS40MzMxLTAuNzE3NTYtMC4zNDI5Mi0xLjQzODQgMC40MzQzMS0xLjAyNDQgMS4xMTU0IDAuMTI5NDMgMC4yNDIgMC40MTM0NCAwLjMzNzYgMC42NzQxNSAwLjMxNzY5eiIgZmlsbD0iI2ZlZmVmZSIvPjxwYXRoIGQ9Im04MS4wNzMgNTkuMTRjMS41NTYyIDAuMTQ1MDUgMy4xNzIyIDAuMjU4ODEgNC42OTUyLTAuMTQ5NjMgMC45OTEyNC0wLjA0MTE0IDAuODE3NTYtMS41OTc2LTAuMTYwMjctMS4yMzg0LTEuMTIwMSAwLjM5OTAxLTIuMzQzNiAwLjM0NTYtMy41MTMzIDAuMjY2MzItMi4wODI4LTAuMjA3MzUtNC4wNTI0LTAuOTY4NTQtNS45ODE5LTEuNzM0My0wLjk0OTM2LTAuMTM0Mi0wLjgzMzI2IDEuMTIxLTAuMDQzMDUgMS4yODM1IDEuMzczNiAwLjYzNzc2IDIuNzM4MyAxLjMzODYgMy45MDMzIDIuMzIyNyAxLjM1OTYgMS4wNTU5IDIuMjgxOCAyLjUzMDEgMy40NTA1IDMuNzcxNCAwLjU4MzM0IDAuNjM2NTYgMS4wNjExIDEuMzYwMyAxLjQ4NTUgMi4xMDYxIDAuNzQ3OTYgMC42MzcxNiAxLjI4ODEtMC42NzUyNCAwLjY3ODMxLTEuMTg4OC0wLjc3NDQyLTEuNDIwOS0xLjk5OTgtMi41MTU0LTMuMDIyMS0zLjc1MDktMC40OTQzLTAuNTYyOTItMS4wMTI0LTEuMTI1MS0xLjQ5MjMtMS42ODgxem0tNDcuODM0IDUuNDE4Yy0yLjI5MTggMC4xNDAzNC00Ljc0NjYgMC4yODk4Mi02LjY3MzkgMS42ODYzLTAuODk0NzQgMC42Mzc3Mi0xLjY5NzUgMS40NzgxLTIuMTEyNCAyLjUwNzQtMC4xMzQxNSAwLjg5NjQgMS4xNDk1IDAuODc0NDMgMS4yNjE0IDAuMDU0MzQgMC45OTU4OC0xLjY1MDQgMi44NzIyLTIuNTMxOCA0LjczMDYtMi43Njk4IDEuMjc5MS0wLjE3MDE0IDIuNTY4Mi0wLjMwNjkyIDMuODU5OC0wLjI2NTA4IDAuOTY1OTQtMC4wMzM2NiAwLjQ2NjE0LTEuNTAzNi0wLjM3MzE2LTEuMjEyNy0wLjIzMSAwLjAwOTktMC40NjEzOSAwLjAwOTktMC42OTIyOS00Ljg0ZS00em0tNS42NDgtMi4zOWMwLjA0NTY4IDAuOTQxNDcgMC45Njk4MiAwLjQzMjIyIDEuNDU5IDAuMTk1ODQgMS4yMjgzLTAuNDUyNjkgMi41NTQ5LTAuNDUyMTcgMy44Mjg4LTAuNjcwMDIgMC44MTI1NC0wLjU5NzE4LTAuMTc1NTItMS4zNTgxLTAuODc2NzYtMS4wNDkxLTEuNDM3OCAwLjA0NTQ3LTIuOTYxOCAwLjE4ODI5LTQuMTk1NiAwLjk5NDUtMC4xNjE2OCAwLjEyNDEtMC4yNDM0NSAwLjMyNjY2LTAuMjE1NDQgMC41Mjg3OHoiIGZpbGw9IiMwYTEzMGEiLz48L2c+PC9nPjxkZWZzPjxjbGlwUGF0aCBpZD0iYSI+PHBhdGggZD0iTTAgMGgxMjB2MTIwSDB6IiBmaWxsPSIjZmZmIi8+PC9jbGlwUGF0aD48L2RlZnM+PC9zdmc+Cg=="; initial_balances = vec {}; maximum_number_of_accounts = null; accounts_overflow_trim_quantity = null }; git_commit_hash = "7fbb84aad7188d1d5b3e17b170997c29d1598cb8";  ledger_compressed_wasm_hash = "4ca82938d223c77909dcf594a49ea72c07fd513726cfa7a367dd0be0d6abc679"; index_compressed_wasm_hash = "55dd5ea22b65adf877cea893765561ae290b52e7fdfdc043b5c18ffbaaa78f33"; }})'
```

* [`0x6982508145454ce325ddbe47a25d4ec3d2311933`](https://etherscan.io/token/0x6982508145454ce325ddbe47a25d4ec3d2311933) is the address of the PEPE smart contract on Ethereum Mainnet.
* [`sv3dd-oaaaa-aaaar-qacoa-cai`](https://dashboard.internetcomputer.org/canister/sv3dd-oaaaa-aaaar-qacoa-cai) is the ckETH minter canister.
* The fee collector is the 0000000000000000000000000000000000000000000000000000000000000fee subaccount of the minter canister.
* The transfer fee is `1000_000_000_000_000_000_000`, corresponding approximately to 0.011 USD, roughly in the same ballpark as ckUSDC transfer fee.
* The ledger compressed wasm hash `4ca82938d223c77909dcf594a49ea72c07fd513726cfa7a367dd0be0d6abc679` and the index compressed wasm hash `55dd5ea22b65adf877cea893765561ae290b52e7fdfdc043b5c18ffbaaa78f33` are the version that will be used by the orchestrator to spawn off the ckPEPE ledger and index, respectively. This is exactly the same version as used by the ckUSDC ledger and index that were created with the proposal [129750](https://dashboard.internetcomputer.org/proposal/129750) at commit `4472b0064d347a88649beb526214fde204f906fb`.

## Release Notes

No changes to the ckERC20 ledger suite orchestrator canister codebase, since this proposal uses the same version `7fbb84aad7188d1d5b3e17b170997c29d1598cb8` as the previous proposal ([130395](https://dashboard.internetcomputer.org/proposal/130395)).

## Wasm Verification

Verify that the hash of the gzipped WASM matches the proposed hash.

```
git fetch
git checkout 7fbb84aad7188d1d5b3e17b170997c29d1598cb8
./gitlab-ci/container/build-ic.sh -c
sha256sum ./artifacts/canisters/ic-ledger-suite-orchestrator-canister.wasm.gz
```

Verify that the hash of the gzipped WASM for the ledger and index match the proposed hash.

```
git fetch
git checkout 4472b0064d347a88649beb526214fde204f906fb
./gitlab-ci/container/build-ic.sh -c
sha256sum ./artifacts/canisters/ic-icrc1-ledger-u256.wasm.gz
sha256sum ./artifacts/canisters/ic-icrc1-index-ng-u256.wasm.gz
```