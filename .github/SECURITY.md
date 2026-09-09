# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.14.0+ | ✅                 |
| ≤ 1.13.x | ⚠️  仅安全更新    |

## Reporting a Vulnerability

Please report security vulnerabilities privately via email to **frank@huilinsh.cn**.
Do NOT open a public GitHub issue for security-sensitive findings.

You will receive a response within **48 hours** with an expected timeline for resolution.

## Current Security Measures (V3)

- **Ed25519 signature chain**: Catalog (`/api/plugins`) and self-update (`/api/updater/latest`) responses are signed with Ed25519 on the server. The desktop client embeds the public key at compile time (`include_bytes!("../keys/ed25519-public.bin")`) and verifies each response. Signature failure → catalog degraded to disk cache (or rejected if no cache), self-update blocked.
- **Key rotation**: Private keys are never committed. A new keypair is generated each rotation; old blobs are purged from git history with `filter-repo` + reflog expiry + aggressive GC.
- **Pre-commit / pre-push hooks**: Prevent accidental inclusion of private keys in commits or pushes.
- **JWT fail-fast**: `DSH_JWT_SECRET` is mandatory in production. Server refuses to start without it.
- **Cookie security**: `Secure` flag enforced in production; `HttpOnly` + `SameSite=Lax` everywhere.
- **Rate limiting**: Login/register endpoints limited by IP + email (dual-axis, 15min/1h windows).
- **X-Real-IP trust**: `clientIp()` prefers `X-Real-IP` (set by Nginx, non-spooferable) over `X-Forwarded-For`.

## Attack Surface

The following paths are in scope for security review:

- `src-tauri/src/catalog.rs` — signature verification
- `src-tauri/src/main.rs` — self-update flow (check_self_update, self_update)
- `website/server/api/plugins/index.get.ts` — response signing
- `website/server/api/updater/latest.get.ts` — response signing
- `website/server/utils/auth.ts` — JWT签发/校验/密码哈希
- `website/server/api/auth/**/*.ts` — login/register/callback/logout
- `proxy-server/` — Go reverse proxy (not part of this repo; see [proxy-server repo](https://github.com/Frank-NF/dsh-proxy))

## Acknowledgments

Thank you to all researchers who responsibly disclose vulnerabilities.
