use anyhow::Result;
use std::process::{Command, Output};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const DEFAULT_EXTERNAL_COMMAND_TIMEOUT_SECS: u64 = 120;

/// Return the current Unix epoch in seconds.
///
/// This is the single, canonical implementation — **do not** duplicate
/// this helper in other modules.
pub fn now_epoch_secs() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

/// Truncate `input` to at most `max_chars` Unicode characters, stripping
/// control characters and appending `…` when truncated.
pub fn truncate_with_ellipsis(input: &str, max_chars: usize) -> String {
    let clean: String = input.chars().filter(|c| !c.is_control()).collect();
    if clean.chars().count() > max_chars {
        let mut s: String = clean.chars().take(max_chars).collect();
        s.push('…');
        s
    } else {
        clean
    }
}

pub fn pid_alive(pid: u32) -> bool {
    if cfg!(windows) {
        // On Windows, the simplest way is to try and open the process handle.
        // For now, since we are using fs2 for the actual locking, we can return true
        // and let the try_lock_exclusive failure handle the "alive" check.
        // If we really need to check another process's health, we'd use winapi or tasklist.
        true
    } else {
        let mut cmd = Command::new("kill");
        cmd.arg("-0").arg(pid.to_string());
        let Ok(output) = run_command_with_optional_timeout(&mut cmd, Some(2)) else {
            return false;
        };
        output.status.success()
    }
}

pub fn run_command_with_timeout(cmd: &mut Command) -> Result<Output> {
    run_command_with_optional_timeout(cmd, Some(DEFAULT_EXTERNAL_COMMAND_TIMEOUT_SECS))
}

pub fn run_command_with_optional_timeout(
    cmd: &mut Command,
    timeout_secs: Option<u64>,
) -> Result<Output> {
    let Some(timeout_secs) = timeout_secs else {
        return Ok(cmd.output()?);
    };
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn()?;
    let started = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return Ok(child.wait_with_output()?);
        }
        if started.elapsed() >= Duration::from_secs(timeout_secs) {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!("command timed out after {}s", timeout_secs);
        }
        thread::sleep(Duration::from_millis(50));
    }
}
