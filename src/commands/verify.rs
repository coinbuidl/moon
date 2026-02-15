use anyhow::Result;

use crate::commands::CommandReport;
use crate::commands::status;
use crate::openclaw::doctor;
use crate::openclaw::gateway;

#[derive(Debug, Clone, Default)]
pub struct VerifyOptions {
    pub strict: bool,
}

pub fn run(opts: &VerifyOptions) -> Result<CommandReport> {
    let mut report = status::run()?;
    report.command = "verify".to_string();

    if !gateway::openclaw_available() {
        report.issue("openclaw binary unavailable in PATH/OPENCLAW_BIN");
        return Ok(report);
    }

    if let Err(err) = doctor::run_full_doctor() {
        report.issue(format!("doctor failed: {err}"));
    } else {
        report.detail("doctor: ok".to_string());
    }

    if opts.strict && !report.ok {
        report.issue("strict verify failed");
    }

    Ok(report)
}
