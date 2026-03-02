use anyhow::Result;

use crate::commands::CommandReport;
use crate::commands::moon_stop;
use crate::commands::moon_watch::{self, MoonWatchOptions};

pub fn run() -> Result<CommandReport> {
    let mut report = CommandReport::new("restart");

    report.detail("stopping existing watcher daemon".to_string());
    let stop_report = moon_stop::run()?;
    let stop_ok = stop_report.ok;
    report.merge(stop_report);
    if !stop_ok {
        report.issue("restart aborted: stop failed");
        return Ok(report);
    }

    report.detail("starting new watcher daemon".to_string());
    let watch_report = moon_watch::run(&MoonWatchOptions {
        once: false,
        daemon: true,
        dry_run: false,
    })?;
    report.merge(watch_report);

    Ok(report)
}
