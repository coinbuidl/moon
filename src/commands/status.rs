use anyhow::Result;
use serde_json::Value;

use crate::commands::CommandReport;
use crate::openclaw::config;
use crate::openclaw::gateway;
use crate::openclaw::paths::resolve_paths;
use crate::openclaw::plugin_verify;

#[derive(Debug, Clone, Default)]
pub struct StatusSnapshot {
    pub plugin_enabled: bool,
    pub reserve_tokens_floor: bool,
    pub max_history_share: bool,
    pub context_pruning_mode: bool,
    pub context_pruning_soft_trim: bool,
    pub plugin_max_tokens: bool,
    pub plugin_max_chars: bool,
    pub plugin_max_retained_bytes: bool,
    pub plugin_read_profile_tokens: bool,
}

fn path_exists(root: &Value, path: &[&str]) -> bool {
    let mut cursor = root;
    for part in path {
        let Some(next) = cursor.get(*part) else {
            return false;
        };
        cursor = next;
    }
    true
}

fn path_value<'a>(root: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut cursor = root;
    for part in path {
        let next = cursor.get(*part)?;
        cursor = next;
    }
    Some(cursor)
}

pub fn config_snapshot(root: &Value, plugin_id: &str) -> StatusSnapshot {
    StatusSnapshot {
        plugin_enabled: root
            .get("plugins")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.get(plugin_id))
            .and_then(|v| v.get("enabled"))
            .and_then(Value::as_bool)
            .unwrap_or(false),
        reserve_tokens_floor: path_exists(
            root,
            &["agents", "defaults", "compaction", "reserveTokensFloor"],
        ),
        max_history_share: path_exists(
            root,
            &["agents", "defaults", "compaction", "maxHistoryShare"],
        ),
        context_pruning_mode: path_exists(root, &["agents", "defaults", "contextPruning", "mode"]),
        context_pruning_soft_trim: path_exists(
            root,
            &[
                "agents",
                "defaults",
                "contextPruning",
                "softTrim",
                "maxChars",
            ],
        ),
        plugin_max_tokens: path_exists(
            root,
            &["plugins", "entries", plugin_id, "config", "maxTokens"],
        ),
        plugin_max_chars: path_exists(
            root,
            &["plugins", "entries", plugin_id, "config", "maxChars"],
        ),
        plugin_max_retained_bytes: path_exists(
            root,
            &[
                "plugins",
                "entries",
                plugin_id,
                "config",
                "maxRetainedBytes",
            ],
        ),
        plugin_read_profile_tokens: path_exists(
            root,
            &[
                "plugins",
                "entries",
                plugin_id,
                "config",
                "tools",
                "read",
                "maxTokens",
            ],
        ),
    }
}

pub fn run() -> Result<CommandReport> {
    let paths = resolve_paths()?;
    let mut report = CommandReport::new("status");

    report.detail(format!("state_dir={}", paths.state_dir.display()));
    report.detail(format!("config_path={}", paths.config_path.display()));
    report.detail(format!("plugin_dir={}", paths.plugin_dir.display()));

    let cfg = config::read_config_value(&paths)?;
    let snapshot = config_snapshot(&cfg, &paths.plugin_id);

    let verify = plugin_verify::verify_plugin(&paths)?;

    report.detail(format!("plugin_present_on_disk={}", verify.present_on_disk));
    report.detail(format!(
        "plugin_listed_by_openclaw={}",
        verify.listed_by_openclaw
    ));
    report.detail(format!(
        "plugin_loaded_by_openclaw={}",
        verify.loaded_by_openclaw
    ));
    report.detail(format!(
        "plugin_assets_match_local={}",
        verify.assets_match_local
    ));
    report.detail(format!("plugin_enabled={}", snapshot.plugin_enabled));

    if let Some(v) = path_value(
        &cfg,
        &[
            "plugins",
            "entries",
            &paths.plugin_id,
            "config",
            "maxTokens",
        ],
    ) {
        report.detail(format!("plugin_config.maxTokens={v}"));
    }
    if let Some(v) = path_value(
        &cfg,
        &["plugins", "entries", &paths.plugin_id, "config", "maxChars"],
    ) {
        report.detail(format!("plugin_config.maxChars={v}"));
    }
    if let Some(v) = path_value(
        &cfg,
        &[
            "plugins",
            "entries",
            &paths.plugin_id,
            "config",
            "maxRetainedBytes",
        ],
    ) {
        report.detail(format!("plugin_config.maxRetainedBytes={v}"));
    }

    if !snapshot.reserve_tokens_floor {
        report.issue("missing agents.defaults.compaction.reserveTokensFloor");
    }
    if !snapshot.max_history_share {
        report.issue("missing agents.defaults.compaction.maxHistoryShare");
    }
    if !snapshot.context_pruning_mode {
        report.issue("missing agents.defaults.contextPruning.mode");
    }
    if !snapshot.context_pruning_soft_trim {
        report.issue("missing agents.defaults.contextPruning.softTrim.maxChars");
    }
    if !snapshot.plugin_max_tokens {
        report.issue("missing plugins.entries.oc-token-optim.config.maxTokens");
    }
    if !snapshot.plugin_max_chars {
        report.issue("missing plugins.entries.oc-token-optim.config.maxChars");
    }
    if !snapshot.plugin_max_retained_bytes {
        report.issue("missing plugins.entries.oc-token-optim.config.maxRetainedBytes");
    }
    if !snapshot.plugin_read_profile_tokens {
        report.issue("missing plugins.entries.oc-token-optim.config.tools.read.maxTokens");
    }
    if !verify.present_on_disk {
        report.issue("plugin files missing on disk");
    }
    if !verify.assets_match_local {
        report.issue("installed plugin assets drift from local package assets");
    }
    if gateway::openclaw_available() && !verify.listed_by_openclaw {
        report.issue("plugin not listed by `openclaw plugins list --json`");
    }
    if gateway::openclaw_available() && !verify.loaded_by_openclaw {
        report.issue("plugin is listed but not loaded");
    }
    if !snapshot.plugin_enabled {
        report.issue("plugin entry is not enabled in config");
    }

    Ok(report)
}
