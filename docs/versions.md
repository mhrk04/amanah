# Toolchain

Recorded for the scaffold. `anchor` CLI and `solana` CLI were not installed in the environment that generated this file, so `anchor build` was not run. `cargo check` uses the host compiler below.

| Tool | Pinned | Notes |
|---|---|---|
| Rust | 1.89.0 | Anchor MSRV (`rust-toolchain.toml`). The image default was 1.83, which is too old. |
| anchor-lang / anchor-spl | =1.2.1 | crates.io. Features: `init-if-needed`; `token`, `token_2022`, `associated_token`. |
| Anchor CLI | 1.2.1 | Named in `Anchor.toml`. Install with avm in T02. Not on PATH here. |
| Solana CLI | 4.1.2 | Anchor 1.2.0 release notes. Design §13 item 8: let avm confirm the 1.2.1 pairing. |
| Node | 22 | LTS. |
| pnpm | 10.33.3 | `packageManager` in the root `package.json`. |
| @anchor-lang/core | 1.2.1 | Not `@coral-xyz/anchor`. |
| @solana/web3.js | 1.99.0 | |
| @solana/kit | 8.4.0 | Keeper. |
| @chainlink/cre-sdk | 1.23.0 | CRE CLI 1.36.0 is a separate install (T02). |
| Next.js | 16.3.8 | Design names App Router, not this exact release. |
| TypeScript | 5.9.3 | |
