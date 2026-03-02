# MOON Bootstrap Guide

Welcome to the MOON system. To ensure a smooth installation and stable context management, follow this protocol.

## 1. Environment Preparation
Before installation, you MUST define your workspace boundaries. Export these in your shell or add them to your `.env` file:

```bash
# The absolute path to your OpenClaw workspace
export MOON_HOME="$HOME/.openclaw_workspace"

# Optional override for OpenClaw binary path.
# If unset, moon resolves `openclaw` from PATH.
export OPENCLAW_BIN="/absolute/path/to/openclaw"
```

Validation:

```bash
command -v openclaw
```

## 2. Provenance Handshake
MOON requires a "provenance" registration with OpenClaw to authorize context pruning.

1. **Build**: `cargo build --release`
2. **Install**: `moon install` (This registers the plugin, sets up internal paths, and on macOS enables `launchd` auto-start + auto-restart for the watcher daemon when run from an installed binary).
3. **Verify**: `moon verify --strict` (Ensure all checks are GREEN).
4. **Inspect config**: `moon config --show` (confirm resolved runtime values).

## 3. Dependency Check: qmd
MOON uses `qmd` for vector indexing and recall.
- Ensure `qmd` is installed and accessible.
- Run `moon status --json` to verify that `qmd_bin` is correctly detected.

## 4. The Watcher Daemon
The Watcher is the "brain" of the system. It handles archival, compaction, and distillation.

- **Default start (macOS)**: `moon install` (registers + starts `com.moon.watch`)
- **Manual foreground start**: `moon watch --daemon`
- **Check Runtime Paths**: `moon status`
- **Check Daemon/State Health**: `moon health`
- **Audit Logs**: Monitor `$MOON_HOME/moon/logs/audit.log` for activity.
- **Dry-run one cycle safely**: `moon watch --once --dry-run`

Workspace safety:

1. Mutating commands enforce workspace CWD boundaries.
2. Use global `--allow-out-of-bounds` only when intentionally operating outside the workspace root.

## 5. Embedding Strategy (Large Backlogs)
If you have a massive existing session history (e.g., >10,000 chunks):
- **Bounded only**: MOON requires bounded embed (`qmd embed --max-docs`) for verifiable progress.
- **Auto watcher embed**: Keep `[embed].mode = "auto"` (legacy aliases normalize to `auto`). Watcher runs embed near cycle end with cooldown + pending-doc gates.
- **Manual Sprints**: Use `moon embed --max-docs 20` for controlled, verifiable progress.

## 6. Skill File Placement (Required)
MOON ships two role-scoped skill guides in this repo root:

- `SKILL.md`: admin/operator scope (install, verify, repair, watcher lifecycle).
- `SKILL_SUBAGENT.md`: least-privilege sub-agent scope (recall, distill, bounded embed).

Keep these files in the repo root as source-of-truth:

- `<moon-repo>/SKILL.md`
- `<moon-repo>/SKILL_SUBAGENT.md`

If your agent runtime loads skills from `$CODEX_HOME/skills/<skill-name>/SKILL.md`,
install both files to explicit skill directories:

```bash
MOON_REPO="/absolute/path/to/moon"
SKILLS_HOME="${CODEX_HOME:-$HOME/.codex}/skills"

mkdir -p "$SKILLS_HOME/moon-admin" "$SKILLS_HOME/moon-subagent"
cp "$MOON_REPO/SKILL.md" "$SKILLS_HOME/moon-admin/SKILL.md"
cp "$MOON_REPO/SKILL_SUBAGENT.md" "$SKILLS_HOME/moon-subagent/SKILL.md"
```

## 7. Sub-agent Memory Access
When delegating memory tasks, give sub-agents only the sub-agent skill
(`moon-subagent`). Do not grant admin/operator skill (`moon-admin`) to
sub-agents.

Example workspace rule (`AGENTS.md`):

```md
- Primary operator agent may use `moon-admin`.
- Sub-agents must use `moon-subagent` only.
```

---
*Follow these steps to achieve a self-healing, memory-aware workspace. - Lilac* ✨💕
