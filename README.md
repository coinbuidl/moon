# oc-token-optim

A reinstallable OpenClaw optimization plugin package.

`oc-token-optim` started with Stage 1 (context/persist compaction baseline) and now includes Stage 2 token-aware optimization.

## What It Does

1. Installs and syncs the `oc-token-optim` OpenClaw plugin into `~/.openclaw/extensions/oc-token-optim`.
2. Ensures plugin enablement in OpenClaw config.
3. Applies conservative config hardening defaults (idempotent by default).
4. Provides one-command post-upgrade recovery (`post-upgrade`) after OpenClaw upgrades.

## For Lilac Agent (Fast Path)

Run these in `/Users/lilac/gh/oc-token-optim`:

```bash
cargo run -- install
cargo run -- verify
cargo run -- post-upgrade --json
```

After any OpenClaw upgrade, run:

```bash
cargo run -- post-upgrade
```

## CLI Commands

```bash
cargo run -- status
cargo run -- install
cargo run -- install --dry-run
cargo run -- install --force
cargo run -- verify
cargo run -- repair
cargo run -- post-upgrade
cargo run -- post-upgrade --json
```

## Command Behavior

### `install`

1. Syncs plugin assets from `assets/plugin` to OpenClaw extensions dir.
2. Enables `plugins.entries.oc-token-optim.enabled = true`.
3. Applies missing defaults without overwriting user-set values.
4. Use `--force` to overwrite targeted defaults.

### `verify`

Checks:

1. Plugin files exist on disk.
2. Plugin appears in `openclaw plugins list --json`.
3. Plugin is enabled in config.
4. Required optimization config keys exist.
5. `openclaw doctor` succeeds.

### `repair`

1. Force reinstall + repatch.
2. Restart gateway (with stop/start fallback).
3. Re-verify.

### `post-upgrade`

1. Install/sync.
2. Restart gateway.
3. Verify.
4. Auto-fallback to `repair` if verify fails.

## Stage 2 (Token Optimization) Implemented

Implemented plugin config keys:

1. `plugins.entries.oc-token-optim.config.maxTokens`
2. `plugins.entries.oc-token-optim.config.maxChars`
3. `plugins.entries.oc-token-optim.config.maxRetainedBytes`
4. `plugins.entries.oc-token-optim.config.tools.<tool>.maxTokens`
5. `plugins.entries.oc-token-optim.config.tools.<tool>.maxChars`

Implementation details and rollout steps are documented in `implementation_plan.md`.

## Upgrade-Safe Reinstall Design

1. Source of truth is this repository (`assets/plugin/*`).
2. Install sync is idempotent and drift-aware.
3. `post-upgrade` re-applies install/verify after OpenClaw updates.
4. `repair` is the force fallback path.

## Development

```bash
cargo fmt --all
cargo check
cargo clippy -- -D warnings
cargo test
```

## Troubleshooting

1. `openclaw` not found:
   - Ensure `openclaw` is in `PATH`, or set `OPENCLAW_BIN`.
2. Config parse failure:
   - Fix invalid `~/.openclaw/openclaw.json` syntax, then rerun `install`.
3. Verify failed after upgrade:
   - Run `cargo run -- repair`.

## Paths

1. Repo: `/Users/lilac/gh/oc-token-optim`
2. Plugin install target: `~/.openclaw/extensions/oc-token-optim`
3. OpenClaw config (default): `~/.openclaw/openclaw.json`
