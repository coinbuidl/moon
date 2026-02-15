use anyhow::Result;

use crate::commands::CommandReport;
use crate::commands::install::{self, InstallOptions};
use crate::commands::verify::{self, VerifyOptions};
use crate::openclaw::gateway;

#[derive(Debug, Clone, Default)]
pub struct RepairOptions {
    pub force: bool,
}

pub fn run(opts: &RepairOptions) -> Result<CommandReport> {
    let mut report = CommandReport::new("repair");
    if opts.force {
        report.detail("force mode requested".to_string());
    }

    if !gateway::openclaw_available() {
        report.issue("openclaw binary unavailable in PATH/OPENCLAW_BIN");
        return Ok(report);
    }

    let install_report = install::run(&InstallOptions {
        force: true,
        dry_run: false,
        apply: true,
    })?;
    report.details.extend(install_report.details);
    report.issues.extend(install_report.issues);
    if !install_report.ok {
        report.ok = false;
    }

    if let Err(err) = gateway::run_gateway_restart(2) {
        report.issue(format!("gateway restart failed: {err}"));
        if let Err(fallback_err) = gateway::run_gateway_stop_start() {
            report.issue(format!(
                "gateway stop/start fallback failed: {fallback_err}"
            ));
        } else {
            report.detail("gateway stop/start fallback succeeded".to_string());
        }
    } else {
        report.detail("gateway restart succeeded".to_string());
    }

    let verify_report = verify::run(&VerifyOptions { strict: true })?;
    report.details.extend(verify_report.details);
    report.issues.extend(verify_report.issues);
    if !verify_report.ok {
        report.ok = false;
    }

    Ok(report)
}
