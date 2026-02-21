# Project Identity Transition Report: `oc-token-optim` ➔ `MOON`

This report outlines the technical and operational implications of renaming the project identity from its legacy identifier (`oc-token-optim`) to its brand identifier (`MOON`).

---

## 1. Executive Summary
Currently, the "MOON" project maintains two identities:
- **Product Name**: MOON (Interface, CLI commands)
- **Technical ID**: `oc-token-optim` (Crate name, OpenClaw Plugin ID, configuration keys)

Renaming the technical ID to match the product name will improve brand cohesion and code maintainability but will introduce a breaking change for existing installations.

---

## 2. Pros (Benefits)

### ✅ Brand Cohesion
Matching the CLI name (`MOON`) with the Plugin ID (`MOON`) eliminates cognitive dissonance for users. Currently, a user runs `MOON install` but sees `oc-token-optim` in their `openclaw plugins list`.

### ✅ Codebase Maintainability
Searching the codebase for the project’s name currently returns two distinct sets of results. Standardizing on `MOON` makes the repository more discoverable for new contributors and AI agents.

### ✅ Configuration Clarity
Users currently configure "MOON" by editing a JSON block labeled `oc-token-optim`. Unifying these terms makes the `openclaw.json` config file self-documenting.

---

## 3. Cons (Risks & Drawbacks)

### ⚠️ Breaking Change (High Impact)
OpenClaw identifies plugins by their string ID. If we change the ID from `oc-token-optim` to `MOON`, any existing OpenClaw installation will immediately stop recognizing the installed plugin.

### ⚠️ Manual Migration Effort
Existing users would be required to:
1. Uninstall `oc-token-optim`.
2. Install the new `MOON` plugin.
3. Manually migrate any custom configuration values from the old JSON key to the new one.

### ⚠️ Environmental Updates
System components like `systemd` services or macOS `LaunchAgents` that were generated using the old name will need to be deleted and re-generated to avoid "command not found" errors or silent failures.

---

## 4. Technical Impact Analysis

If a transition is authorized, the following areas require modification:

| Component | Target Change |
| :--- | :--- |
| **Cargo.toml** | Change `name = "oc-token-optim"` to `name = "moon"`. |
| **Source Code** | Update `PLUGIN_ID` constant in `src/openclaw/paths.rs`. |
| **Plugin Assets** | Rename IDs in `package.json` and `openclaw.plugin.json`. |
| **Commands** | Update `status.rs` and `verify.rs` to validate the new ID. |
| **Test Suite** | Update ~12 test files currently asserting the old ID. |

---

## 5. Recommendation
The transition is **recommended** for the long-term health and clarity of the project. However, to mitigate the breaking change, it should be timed with a major version release (e.g., `v1.0.0`) and accompanied by a "Migration Guide" in the README.
