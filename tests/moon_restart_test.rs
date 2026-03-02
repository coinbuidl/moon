#![cfg(not(windows))]
use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn moon_restart_is_registered_in_help() {
    assert_cmd::cargo::cargo_bin_cmd!("moon")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("restart"));
}

#[test]
fn moon_restart_runs_stop_before_start_attempt() {
    let tmp = tempdir().expect("tempdir");
    let logs_dir = tmp.path().join("moon").join("logs");
    std::fs::create_dir_all(&logs_dir).expect("mkdir logs");

    assert_cmd::cargo::cargo_bin_cmd!("moon")
        .current_dir(tmp.path())
        .env("MOON_LOGS_DIR", &logs_dir)
        .arg("restart")
        .assert()
        .failure()
        .stdout(contains(
            "moon watcher daemon already stopped (lock file not found)",
        ))
        .stdout(contains("starting new watcher daemon"))
        .stdout(contains(
            "CRITICAL: Running the background daemon via `cargo run` is disabled for stability.",
        ));
}
