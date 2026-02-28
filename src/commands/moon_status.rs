use anyhow::Result;

use crate::commands::CommandReport;
use crate::moon::config::{SECRET_ENV_KEYS, masked_env_secret};
use crate::moon::paths::resolve_paths;
use crate::moon::state::state_file_path;

pub fn run() -> Result<CommandReport> {
    let paths = resolve_paths()?;
    let mut report = CommandReport::new("moon-status");

    report.detail(format!("moon_home={}", paths.moon_home.display()));
    report.detail(format!("archives_dir={}", paths.archives_dir.display()));
    report.detail(format!("memory_dir={}", paths.memory_dir.display()));
    report.detail(format!("memory_file={}", paths.memory_file.display()));
    report.detail(format!("logs_dir={}", paths.logs_dir.display()));
    report.detail(format!("state_file={}", state_file_path(&paths).display()));
    report.detail(format!(
        "openclaw_sessions_dir={}",
        paths.openclaw_sessions_dir.display()
    ));
    report.detail(format!("qmd_bin={}", paths.qmd_bin.display()));
    report.detail(format!("qmd_db={}", paths.qmd_db.display()));
    for key in SECRET_ENV_KEYS {
        report.detail(format!("secret.{key}={}", masked_env_secret(key)));
    }

    if !paths.archives_dir.exists() {
        report.issue(format!(
            "missing archives dir ({})",
            paths.archives_dir.display()
        ));
    }
    if !paths.memory_dir.exists() {
        report.issue(format!(
            "missing daily memory dir ({})",
            paths.memory_dir.display()
        ));
    }
    if !paths.logs_dir.exists() {
        report.issue(format!(
            "missing moon log dir ({})",
            paths.logs_dir.display()
        ));
    }
    if !paths.memory_file.exists() {
        report.issue(format!(
            "missing long-term memory file ({})",
            paths.memory_file.display()
        ));
    }
    if !paths.openclaw_sessions_dir.exists() {
        report.issue(format!(
            "missing OpenClaw sessions dir ({})",
            paths.openclaw_sessions_dir.display()
        ));
    }
    if !paths.qmd_bin.exists() {
        report.issue(format!("missing qmd binary ({})", paths.qmd_bin.display()));
    }

    Ok(report)
}
