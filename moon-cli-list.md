# Moon CLI (Current)

Generated from local `moon --help` on 2026-03-02.

## Global Usage

```bash
moon [OPTIONS] <COMMAND>
```

Global options:
- `--json`
- `--allow-out-of-bounds`
- `-h, --help`

## Commands

- `install`
- `verify`
- `repair`
- `status`
- `stop`
- `restart`
- `snapshot`
- `index`
- `watch`
- `embed`
- `recall`
- `distill`
- `config`
- `health`

## Command Options

### `install`
- `--force`
- `--dry-run`
- `--apply <APPLY>` (`true|false`, default `true`)

### `verify`
- `--strict`

### `repair`
- `--force`

### `status`
- no command-specific options

### `stop`
- no command-specific options

### `restart`
- no command-specific options

### `snapshot`
- `--source <SOURCE>`
- `--dry-run`

### `index`
- `--name <NAME>` (default `history`)
- `--dry-run`

### `watch`
- `--once`
- `--daemon`
- `--dry-run`

### `embed`
- `--name <NAME>` (default `history`)
- `--max-docs <MAX_DOCS>` (default `25`)
- `--dry-run`
- `--watcher-trigger`

### `recall`
- `--query <QUERY>` (required)
- `--name <NAME>` (default `history`)
- `--channel-key <CHANNEL_KEY>`

### `distill`
- `--mode <MODE>` (default `norm`)
- `--archive <ARCHIVE>`
- `--file <FILES>` (repeatable)
- `--session-id <SESSION_ID>`
- `--dry-run`

### `config`
- `--show`

### `health`
- no command-specific options

---

Note: all commands also accept global options `--json` and `--allow-out-of-bounds`.
