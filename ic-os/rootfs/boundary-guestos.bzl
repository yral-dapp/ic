"""
Enumerate every rootfs file dependency for Boundary-GuestOS
"""

rootfs_files = {
    Label("boundary-guestos/etc/certificate-syncer/domain.tmpl"): "/etc/certificate-syncer/domain.tmpl",
    Label("boundary-guestos/etc/chrony/chrony.conf"): "/etc/chrony/chrony.conf",
    Label("boundary-guestos/etc/crowdsec/bouncers/crowdsec-firewall-bouncer.yaml"): "/etc/crowdsec/bouncers/crowdsec-firewall-bouncer.yaml",
    Label("boundary-guestos/etc/crypttab"): "/etc/crypttab",
    Label("boundary-guestos/etc/csr.conf"): "/etc/csr.conf",
    Label("boundary-guestos/etc/default/locale"): "/etc/default/locale",
    Label("boundary-guestos/etc/default/node_exporter"): "/etc/default/node_exporter",
    Label("boundary-guestos/etc/fstab"): "/etc/fstab",
    Label("boundary-guestos/etc/geoip/EMPTY_NOTICE.md"): "/etc/geoip/EMPTY_NOTICE.md",
    Label("boundary-guestos/etc/geoip/GeoLite2-Country.mmdb"): "/etc/geoip/GeoLite2-Country.mmdb",
    Label("boundary-guestos/etc/hostname"): "/etc/hostname",
    Label("boundary-guestos/etc/hosts"): "/etc/hosts",
    Label("boundary-guestos/etc/initramfs-tools/hooks/veritysetup"): "/etc/initramfs-tools/hooks/veritysetup",
    Label("boundary-guestos/etc/initramfs-tools/initramfs.conf"): "/etc/initramfs-tools/initramfs.conf",
    Label("boundary-guestos/etc/initramfs-tools/modules"): "/etc/initramfs-tools/modules",
    Label("boundary-guestos/etc/initramfs-tools/scripts/init-bottom/set-machine-id"): "/etc/initramfs-tools/scripts/init-bottom/set-machine-id",
    Label("boundary-guestos/etc/initramfs-tools/scripts/init-premount/verity-root"): "/etc/initramfs-tools/scripts/init-premount/verity-root",
    Label("boundary-guestos/etc/logrotate.d/nginx"): "/etc/logrotate.d/nginx",
    Label("boundary-guestos/etc/nftables.conf"): "/etc/nftables.conf",
    Label("boundary-guestos/etc/nginx/conf.d/000-nginx-global.conf"): "/etc/nginx/conf.d/000-nginx-global.conf",
    Label("boundary-guestos/etc/nginx/conf.d/001-rosetta-nginx.conf"): "/etc/nginx/conf.d/001-rosetta-nginx.conf",
    Label("boundary-guestos/etc/nginx/conf.d/002-mainnet-nginx.conf"): "/etc/nginx/conf.d/002-mainnet-nginx.conf",
    Label("boundary-guestos/etc/nginx/ffdhe4096.pem"): "/etc/nginx/ffdhe4096.pem",
    Label("boundary-guestos/etc/nginx/ic_public_key.pem"): "/etc/nginx/ic_public_key.pem",
    Label("boundary-guestos/etc/nginx/includes/cache.conf"): "/etc/nginx/includes/cache.conf",
    Label("boundary-guestos/etc/nginx/includes/error_pages.conf"): "/etc/nginx/includes/error_pages.conf",
    Label("boundary-guestos/etc/nginx/includes/method_get.conf"): "/etc/nginx/includes/method_get.conf",
    Label("boundary-guestos/etc/nginx/includes/method_post.conf"): "/etc/nginx/includes/method_post.conf",
    Label("boundary-guestos/etc/nginx/includes/options.conf"): "/etc/nginx/includes/options.conf",
    Label("boundary-guestos/etc/nginx/includes/proxy_headers.conf"): "/etc/nginx/includes/proxy_headers.conf",
    Label("boundary-guestos/etc/nginx/includes/request_id.conf"): "/etc/nginx/includes/request_id.conf",
    Label("boundary-guestos/etc/nginx/includes/response_headers.conf"): "/etc/nginx/includes/response_headers.conf",
    Label("boundary-guestos/etc/nginx/includes/slice.conf"): "/etc/nginx/includes/slice.conf",
    Label("boundary-guestos/etc/nginx/includes/whitelist_rosetta.conf"): "/etc/nginx/includes/whitelist_rosetta.conf",
    Label("boundary-guestos/etc/nginx/nginx.conf"): "/etc/nginx/nginx.conf",
    Label("boundary-guestos/etc/prober-wallets-exporter/wallets.json"): "/etc/prober-wallets-exporter/wallets.json",
    Label("boundary-guestos/etc/prober/ic_public_key.pem"): "/etc/prober/ic_public_key.pem",
    Label("boundary-guestos/etc/prober/wallets.json"): "/etc/prober/wallets.json",
    Label("boundary-guestos/etc/resolv.conf"): "/etc/resolv.conf",
    Label("boundary-guestos/etc/security/limits.conf"): "/etc/security/limits.conf",
    Label("boundary-guestos/etc/stunnel/stunnel.conf"): "/etc/stunnel/stunnel.conf",
    Label("boundary-guestos/etc/sudoers"): "/etc/sudoers",
    Label("boundary-guestos/etc/sysctl.d/network-tweaks.conf"): "/etc/sysctl.d/network-tweaks.conf",
    Label("boundary-guestos/etc/systemd/journald.conf"): "/etc/systemd/journald.conf",
    Label("boundary-guestos/etc/systemd/resolved.conf.d/fallback.conf"): "/etc/systemd/resolved.conf.d/fallback.conf",
    Label("boundary-guestos/etc/systemd/system/bootstrap-ic-node.service"): "/etc/systemd/system/bootstrap-ic-node.service",
    Label("boundary-guestos/etc/systemd/system/canary-proxy.service"): "/etc/systemd/system/canary-proxy.service",
    Label("boundary-guestos/etc/systemd/system/certificate-issuer.service"): "/etc/systemd/system/certificate-issuer.service",
    Label("boundary-guestos/etc/systemd/system/certificate-syncer.service"): "/etc/systemd/system/certificate-syncer.service",
    Label("boundary-guestos/etc/systemd/system/crowdsec-firewall-bouncer.service"): "/etc/systemd/system/crowdsec-firewall-bouncer.service",
    Label("boundary-guestos/etc/systemd/system/danted.service"): "/etc/systemd/system/danted.service",
    Label("boundary-guestos/etc/systemd/system/generate-dante-config.service"): "/etc/systemd/system/generate-dante-config.service",
    Label("boundary-guestos/etc/systemd/system/generate-network-config.service"): "/etc/systemd/system/generate-network-config.service",
    Label("boundary-guestos/etc/systemd/system/ic-boundary.service"): "/etc/systemd/system/ic-boundary.service",
    Label("boundary-guestos/etc/systemd/system/icx-proxy.service"): "/etc/systemd/system/icx-proxy.service",
    Label("boundary-guestos/etc/systemd/system/logrotate.timer"): "/etc/systemd/system/logrotate.timer",
    Label("boundary-guestos/etc/systemd/system/nftables.service.d/override.conf"): "/etc/systemd/system/nftables.service.d/override.conf",
    Label("boundary-guestos/etc/systemd/system/nginx.service.d/override.conf"): "/etc/systemd/system/nginx.service.d/override.conf",
    Label("boundary-guestos/etc/systemd/system/node_exporter.service"): "/etc/systemd/system/node_exporter.service",
    Label("boundary-guestos/etc/systemd/system/prober.service"): "/etc/systemd/system/prober.service",
    Label("boundary-guestos/etc/systemd/system/prober-wallets-exporter.service"): "/etc/systemd/system/prober-wallets-exporter.service",
    Label("boundary-guestos/etc/systemd/system/setup-canary-proxy.service"): "/etc/systemd/system/setup-canary-proxy.service",
    Label("boundary-guestos/etc/systemd/system/setup-certificate-issuer.service"): "/etc/systemd/system/setup-certificate-issuer.service",
    Label("boundary-guestos/etc/systemd/system/setup-certificate-syncer.service"): "/etc/systemd/system/setup-certificate-syncer.service",
    Label("boundary-guestos/etc/systemd/system/setup-conf-encryption.service"): "/etc/systemd/system/setup-conf-encryption.service",
    Label("boundary-guestos/etc/systemd/system/setup-crowdsec.service"): "/etc/systemd/system/setup-crowdsec.service",
    Label("boundary-guestos/etc/systemd/system/setup-encryption.service"): "/etc/systemd/system/setup-encryption.service",
    Label("boundary-guestos/etc/systemd/system/setup-ic-boundary.service"): "/etc/systemd/system/setup-ic-boundary.service",
    Label("boundary-guestos/etc/systemd/system/setup-icx-proxy.service"): "/etc/systemd/system/setup-icx-proxy.service",
    Label("boundary-guestos/etc/systemd/system/setup-lvs.service"): "/etc/systemd/system/setup-lvs.service",
    Label("boundary-guestos/etc/systemd/system/setup-nftables.service"): "/etc/systemd/system/setup-nftables.service",
    Label("boundary-guestos/etc/systemd/system/setup-nginx.service"): "/etc/systemd/system/setup-nginx.service",
    Label("boundary-guestos/etc/systemd/system/setup-prober.service"): "/etc/systemd/system/setup-prober.service",
    Label("boundary-guestos/etc/systemd/system/setup-var-log.service"): "/etc/systemd/system/setup-var-log.service",
    Label("boundary-guestos/etc/systemd/system/setup-vector.service"): "/etc/systemd/system/setup-vector.service",
    Label("boundary-guestos/etc/systemd/system/systemd-journal-gatewayd-shim.service"): "/etc/systemd/system/systemd-journal-gatewayd-shim.service",
    Label("boundary-guestos/etc/systemd/system/vector.service"): "/etc/systemd/system/vector.service",
    Label("boundary-guestos/etc/tmpfiles.d/ic-node.conf"): "/etc/tmpfiles.d/ic-node.conf",
    Label("boundary-guestos/etc/vector/vector.yaml"): "/etc/vector/vector.yaml",
    Label("boundary-guestos/opt/ic/bin/bootstrap-ic-node.sh"): "/opt/ic/bin/bootstrap-ic-node.sh",
    Label("boundary-guestos/opt/ic/bin/exec_condition.shlib"): "/opt/ic/bin/exec_condition.shlib",
    Label("boundary-guestos/opt/ic/bin/generate-dante-config.sh"): "/opt/ic/bin/generate-dante-config.sh",
    Label("boundary-guestos/opt/ic/bin/generate-network-config.sh"): "/opt/ic/bin/generate-network-config.sh",
    Label("boundary-guestos/opt/ic/bin/helpers.shlib"): "/opt/ic/bin/helpers.shlib",
    Label("boundary-guestos/opt/ic/bin/manageboot.sh"): "/opt/ic/bin/manageboot.sh",
    Label("boundary-guestos/opt/ic/bin/setup-canary-proxy.sh"): "/opt/ic/bin/setup-canary-proxy.sh",
    Label("boundary-guestos/opt/ic/bin/setup-certificate-issuer.sh"): "/opt/ic/bin/setup-certificate-issuer.sh",
    Label("boundary-guestos/opt/ic/bin/setup-certificate-syncer.sh"): "/opt/ic/bin/setup-certificate-syncer.sh",
    Label("boundary-guestos/opt/ic/bin/setup-conf-encryption.sh"): "/opt/ic/bin/setup-conf-encryption.sh",
    Label("boundary-guestos/opt/ic/bin/setup-crowdsec.sh"): "/opt/ic/bin/setup-crowdsec.sh",
    Label("boundary-guestos/opt/ic/bin/setup-encryption.sh"): "/opt/ic/bin/setup-encryption.sh",
    Label("boundary-guestos/opt/ic/bin/setup-ic-boundary.sh"): "/opt/ic/bin/setup-ic-boundary.sh",
    Label("boundary-guestos/opt/ic/bin/setup-icx-proxy.sh"): "/opt/ic/bin/setup-icx-proxy.sh",
    Label("boundary-guestos/opt/ic/bin/setup-lvs.sh"): "/opt/ic/bin/setup-lvs.sh",
    Label("boundary-guestos/opt/ic/bin/setup-nftables.sh"): "/opt/ic/bin/setup-nftables.sh",
    Label("boundary-guestos/opt/ic/bin/setup-nginx.sh"): "/opt/ic/bin/setup-nginx.sh",
    Label("boundary-guestos/opt/ic/bin/setup-prober.sh"): "/opt/ic/bin/setup-prober.sh",
    Label("boundary-guestos/opt/ic/bin/setup-var-encryption.sh"): "/opt/ic/bin/setup-var-encryption.sh",
    Label("boundary-guestos/opt/ic/bin/setup-var-log.sh"): "/opt/ic/bin/setup-var-log.sh",
    Label("boundary-guestos/opt/ic/bin/setup-vector.sh"): "/opt/ic/bin/setup-vector.sh",
    Label("boundary-guestos/opt/ic/share/.gitignore"): "/opt/ic/share/.gitignore",
    Label("boundary-guestos/opt/ic/share/ark.pem"): "/opt/ic/share/ark.pem",
    Label("boundary-guestos/var/www/html/451.html"): "/var/www/html/451.html",
    Label("boundary-guestos/var/www/html/uninstall-script.js"): "/var/www/html/uninstall-script.js",

    # consolidated files:
    Label("systemd-generators/boundary-guestos/mount-generator"): "/etc/systemd/system-generators/mount-generator",
    Label("systemd-generators/systemd-gpt-auto-generator"): "/etc/systemd/system-generators/systemd-gpt-auto-generator",
    Label("save-machine-id/save-machine-id.sh"): "/opt/ic/bin/save-machine-id.sh",
    Label("save-machine-id/save-machine-id.service"): "/etc/systemd/system/save-machine-id.service",
    Label("utils/install-upgrade.sh"): "/opt/ic/bin/install-upgrade.sh",
    Label("relabel-machine-id/relabel-machine-id.sh"): "/opt/ic/bin/relabel-machine-id.sh",
    Label("relabel-machine-id/relabel-machine-id.service"): "/etc/systemd/system/relabel-machine-id.service",
    Label("setup-hostname/setup-hostname.sh"): "/opt/ic/bin/setup-hostname.sh",
    Label("setup-hostname/setup-hostname.service"): "/etc/systemd/system/setup-hostname.service",
    Label("retry-ipv6-config/boundary-guestos/retry-ipv6-config.sh"): "/opt/ic/bin/retry-ipv6-config.sh",
    Label("retry-ipv6-config/retry-ipv6-config.service"): "/etc/systemd/system/retry-ipv6-config.service",
    Label("ssh/setup-ssh-keys/setup-ssh-keys.sh"): "/opt/ic/bin/setup-ssh-keys.sh",
    Label("ssh/setup-ssh-keys/setup-ssh-keys.service"): "/etc/systemd/system/setup-ssh-keys.service",
    Label("ssh/setup-ssh-account-keys/boundary-guestos/setup-ssh-account-keys.sh"): "/opt/ic/bin/setup-ssh-account-keys.sh",
    Label("ssh/setup-ssh-account-keys/setup-ssh-account-keys.service"): "/etc/systemd/system/setup-ssh-account-keys.service",
}
