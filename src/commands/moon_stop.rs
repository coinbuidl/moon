use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use crate::commands::CommandReport;
use crate::moon::daemon_lock::{daemon_lock_path, read_daemon_lock_payload};
use crate::moon::paths::resolve_paths;
use crate::moon::util::run_command_with_optional_timeout;

const STOP_TIMEOUT: Duration = Duration::from_secs(8);
const STOP_POLL_INTERVAL: Duration = Duration::from_millis(100);
const COMMAND_TIMEOUT_SECS: u64 = 10;

fn lock_path() -> Result<std::path::PathBuf> {
    let paths = resolve_paths()?;
    Ok(daemon_lock_path(&paths))
}

fn process_alive(pid: u32) -> Result<bool> {
    let mut kill_cmd = Command::new("kill");
    kill_cmd.arg("-0").arg(pid.to_string());
    let kill_out = run_command_with_optional_timeout(&mut kill_cmd, Some(COMMAND_TIMEOUT_SECS))
        .context("failed to probe process state with `kill -0`")?;
    if !kill_out.status.success() {
        return Ok(false);
    }

    let mut ps_cmd = Command::new("ps");
    ps_cmd.arg("-p").arg(pid.to_string()).arg("-o").arg("stat=");
    let ps_out = run_command_with_optional_timeout(&mut ps_cmd, Some(COMMAND_TIMEOUT_SECS))
        .context("failed to inspect process state with `ps`")?;

    if !ps_out.status.success() {
        return Ok(false);
    }

    let proc_state = String::from_utf8_lossy(&ps_out.stdout).trim().to_string();
    if proc_state.starts_with('Z') {
        return Ok(false);
    }

    Ok(true)
}

fn process_command_line(pid: u32) -> Result<String> {
    let mut ps_cmd = Command::new("ps");
    ps_cmd
        .arg("-p")
        .arg(pid.to_string())
        .arg("-o")
        .arg("command=");
    let output = run_command_with_optional_timeout(&mut ps_cmd, Some(COMMAND_TIMEOUT_SECS))
        .context("failed to inspect process command line with `ps`")?;
    if !output.status.success() {
        return Ok(String::new());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn send_sigterm(pid: u32) -> Result<()> {
    let mut kill_cmd = Command::new("kill");
    kill_cmd.arg("-TERM").arg(pid.to_string());
    let out = run_command_with_optional_timeout(&mut kill_cmd, Some(COMMAND_TIMEOUT_SECS))
        .context("failed to send SIGTERM with `kill -TERM`")?;

    if out.status.success() {
        return Ok(());
    }

    if process_alive(pid)? {
        anyhow::bail!("`kill -TERM {pid}` failed and process is still alive");
    }

    Ok(())
}

fn cleanup_lock_file(lock_path: &Path, report: &mut CommandReport) {
    match fs::remove_file(lock_path) {
        Ok(()) => report.detail(format!("removed stale daemon lock {}", lock_path.display())),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => report.detail(format!(
            "failed to remove daemon lock {}: {}",
            lock_path.display(),
            err
        )),
    }
}

pub fn run() -> Result<CommandReport> {
    let mut report = CommandReport::new("moon-stop");
    let lock_path = lock_path()?;
    report.detail(format!("daemon_lock={}", lock_path.display()));

    if !lock_path.exists() {
        report.detail("moon watcher daemon already stopped (lock file not found)".to_string());
        return Ok(report);
    }

    let paths = resolve_paths()?;
    let payload = match read_daemon_lock_payload(&paths) {
        Ok(Some(payload)) => payload,
        Ok(None) => {
            report.detail("moon watcher daemon already stopped (lock payload missing)".to_string());
            return Ok(report);
        }
        Err(err) => {
            report.issue(format!(
                "failed to read daemon lock {}: {err:#}",
                lock_path.display()
            ));
            return Ok(report);
        }
    };
    let pid = payload.pid;
    report.detail(format!("daemon_pid={pid}"));

    if !process_alive(pid)? {
        report.detail(format!("daemon pid {pid} is not running"));
        cleanup_lock_file(&lock_path, &mut report);
        return Ok(report);
    }

    let command_line = process_command_line(pid)?;
    if !command_line.contains("moon-watch --daemon") {
        report.issue(format!(
            "refusing to stop pid {pid}; command does not match moon watcher daemon: {}",
            if command_line.is_empty() {
                "<unknown>".to_string()
            } else {
                command_line
            }
        ));
        return Ok(report);
    }

    send_sigterm(pid)?;
    let deadline = Instant::now() + STOP_TIMEOUT;
    while Instant::now() < deadline {
        if !process_alive(pid)? {
            report.detail(format!("stopped moon watcher daemon pid={pid}"));
            cleanup_lock_file(&lock_path, &mut report);
            return Ok(report);
        }
        thread::sleep(STOP_POLL_INTERVAL);
    }

    report.issue(format!(
        "timed out waiting for daemon pid {pid} to stop after {}s",
        STOP_TIMEOUT.as_secs()
    ));
    Ok(report)
}
