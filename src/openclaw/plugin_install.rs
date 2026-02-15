use crate::assets::{plugin_asset_contents, write_plugin_assets};
use crate::openclaw::gateway;
use crate::openclaw::paths::OpenClawPaths;
use anyhow::Result;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct PluginInstallOutcome {
    pub changed: bool,
    pub path: String,
}

fn plugin_dir_matches_assets(paths: &OpenClawPaths) -> Result<bool> {
    if !paths.plugin_dir.exists() {
        return Ok(false);
    }

    for (name, expected) in plugin_asset_contents() {
        let file = paths.plugin_dir.join(name);
        if !file.exists() {
            return Ok(false);
        }
        let current = fs::read_to_string(&file)?;
        if current != expected {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn install_plugin(paths: &OpenClawPaths, dry_run: bool) -> Result<PluginInstallOutcome> {
    let existed = paths.plugin_dir.exists();
    let matches = plugin_dir_matches_assets(paths)?;
    let needs_update = !matches;

    if !dry_run && needs_update {
        fs::create_dir_all(&paths.extensions_dir)?;
        if paths.plugin_dir.exists() {
            fs::remove_dir_all(&paths.plugin_dir)?;
        }
        write_plugin_assets(&paths.plugin_dir)?;

        if !existed {
            let _ = gateway::try_plugins_install(&paths.plugin_dir);
        }
    }

    Ok(PluginInstallOutcome {
        changed: needs_update,
        path: paths.plugin_dir.display().to_string(),
    })
}
