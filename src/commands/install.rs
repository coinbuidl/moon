use anyhow::Result;

use crate::commands::CommandReport;
use crate::moon::config::load_context_policy_if_explicit_env;
use crate::openclaw::config::{
    ConfigPatchOptions, apply_config_patches, ensure_plugin_enabled, ensure_plugin_install_record,
    read_config_value, write_config_atomic,
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
    let context_policy = load_context_policy_if_explicit_env()?;
    if let Some(policy) = &context_policy {
        report.detail(format!(
            "context.policy=window_mode={:?} prune_mode={:?} compaction_authority={:?}",
            policy.window_mode, policy.prune_mode, policy.compaction_authority
        ));
    } else {
        report.detail(
            "context.policy=legacy (no explicit MOON_CONFIG_PATH/MOON_HOME context section)"
                .to_string(),
        );
    }

    let patch = apply_config_patches(
        &mut cfg,
        &ConfigPatchOptions { force: opts.force },
        &paths.plugin_id,
        context_policy.as_ref(),
    );

    let plugin_patch = ensure_plugin_enabled(&mut cfg, &paths.plugin_id);
    let install_record_patch =
        ensure_plugin_install_record(&mut cfg, &paths.plugin_id, &paths.plugin_dir);

    for key in patch.inserted_paths {
        report.detail(format!("inserted {key}"));
    }
    for key in patch.forced_paths {
        report.detail(format!("forced {key}"));
    }
    for key in patch.removed_paths {
        report.detail(format!("removed {key}"));
    }
    for key in plugin_patch.inserted_paths {
        report.detail(format!("inserted {key}"));
    }
    for key in plugin_patch.forced_paths {
        report.detail(format!("forced {key}"));
    }
    for key in install_record_patch.inserted_paths {
        report.detail(format!("inserted {key}"));
    }
    for key in install_record_patch.forced_paths {
        report.detail(format!("forced {key}"));
    }

    let changed =
        patch.changed || plugin_patch.changed || install_record_patch.changed || plugin.changed;
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
