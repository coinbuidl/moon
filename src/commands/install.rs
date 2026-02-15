use anyhow::Result;

use crate::commands::CommandReport;
use crate::openclaw::config::{
    ConfigPatchOptions, apply_config_patches, ensure_plugin_enabled, read_config_value,
    write_config_atomic,
};
use crate::openclaw::paths::resolve_paths;
use crate::openclaw::plugin_install;

#[derive(Debug, Clone)]
pub struct InstallOptions {
    pub force: bool,
    pub dry_run: bool,
    pub apply: bool,
}

pub fn run(opts: &InstallOptions) -> Result<CommandReport> {
    let paths = resolve_paths()?;
    let mut report = CommandReport::new("install");

    let plugin = plugin_install::install_plugin(&paths, opts.dry_run)?;
    report.detail(format!("plugin_dir={}", plugin.path));
    report.detail(format!("plugin_changed={}", plugin.changed));

    let mut cfg = read_config_value(&paths)?;

    let patch = apply_config_patches(
        &mut cfg,
        &ConfigPatchOptions { force: opts.force },
        &paths.plugin_id,
    );

    let plugin_patch = ensure_plugin_enabled(&mut cfg, &paths.plugin_id);

    for key in patch.inserted_paths {
        report.detail(format!("inserted {key}"));
    }
    for key in patch.forced_paths {
        report.detail(format!("forced {key}"));
    }
    for key in plugin_patch.inserted_paths {
        report.detail(format!("inserted {key}"));
    }
    for key in plugin_patch.forced_paths {
        report.detail(format!("forced {key}"));
    }

    let changed = patch.changed || plugin_patch.changed || plugin.changed;
    if changed && opts.apply && !opts.dry_run {
        let path_written = write_config_atomic(&paths, &cfg)?;
        report.detail(format!("updated config: {path_written}"));
    } else if changed && (opts.dry_run || !opts.apply) {
        report.detail("config changes planned but not applied".to_string());
    } else {
        report.detail("config already satisfied".to_string());
    }

    Ok(report)
}
