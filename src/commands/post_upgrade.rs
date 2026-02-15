use anyhow::Result;

use crate::commands::CommandReport;
use crate::commands::install::{self, InstallOptions};
use crate::commands::repair::{self, RepairOptions};
use crate::commands::verify::{self, VerifyOptions};
use crate::openclaw::gateway;

pub fn run() -> Result<CommandReport> {
    let mut report = CommandReport::new("post-upgrade");

    if !gateway::openclaw_available() {
        report.issue("openclaw binary unavailable in PATH/OPENCLAW_BIN");
        return Ok(report);
    }

    let install_report = install::run(&InstallOptions {
        force: false,
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
        report.detail("post-upgrade verify failed; running automatic repair fallback".to_string());
        let repair_report = repair::run(&RepairOptions { force: true })?;
        report.details.extend(repair_report.details);
        report.issues.extend(repair_report.issues);
        if repair_report.ok {
            report.ok = true;
            report.detail("automatic repair fallback succeeded".to_string());
        } else {
            report.ok = false;
        }
    }

    Ok(report)
}
