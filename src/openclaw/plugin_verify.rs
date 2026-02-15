use crate::assets::plugin_asset_contents;
use anyhow::Result;
use serde_json::Value;
use std::fs;

use crate::openclaw::gateway;
use crate::openclaw::paths::OpenClawPaths;

#[derive(Debug, Clone, Default)]
pub struct PluginVerifyOutcome {
    pub present_on_disk: bool,
    pub listed_by_openclaw: bool,
    pub loaded_by_openclaw: bool,
    pub assets_match_local: bool,
}

pub fn verify_plugin(paths: &OpenClawPaths) -> Result<PluginVerifyOutcome> {
    let present_on_disk = paths.plugin_dir.join("index.js").exists()
        && paths.plugin_dir.join("openclaw.plugin.json").exists()
        && paths.plugin_dir.join("package.json").exists();

    let assets_match_local = if present_on_disk {
        plugin_assets_match_local(paths)
    } else {
        false
    };

    let (listed_by_openclaw, loaded_by_openclaw) = match gateway::plugins_list_json() {
        Ok(raw) => parse_plugins_list_state(&raw, &paths.plugin_id),
        Err(_) => (false, false),
    };

    Ok(PluginVerifyOutcome {
        present_on_disk,
        listed_by_openclaw,
        loaded_by_openclaw,
        assets_match_local,
    })
}

fn plugin_assets_match_local(paths: &OpenClawPaths) -> bool {
    for (name, expected) in plugin_asset_contents() {
        let path = paths.plugin_dir.join(name);
        let Ok(current) = fs::read_to_string(path) else {
            return false;
        };
        if current != expected {
            return false;
        }
    }
    true
}

fn parse_plugins_list_state(raw: &str, plugin_id: &str) -> (bool, bool) {
    let parsed = serde_json::from_str::<Value>(raw);
    let Ok(v) = parsed else {
        return (false, false);
    };

    let arr_opt = v
        .as_array()
        .or_else(|| v.get("plugins").and_then(Value::as_array));

    let Some(arr) = arr_opt else {
        return (false, false);
    };

    for entry in arr {
        let is_match = entry
            .get("id")
            .and_then(Value::as_str)
            .map(|id| id == plugin_id)
            .unwrap_or(false);
        if !is_match {
            continue;
        }
        let loaded = entry
            .get("status")
            .and_then(Value::as_str)
            .map(|status| status == "loaded")
            .unwrap_or(true);
        return (true, loaded);
    }

    (false, false)
}
