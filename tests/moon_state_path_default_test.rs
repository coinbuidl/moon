#![cfg(not(windows))]
use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn status_default_state_path_uses_home_workspace_root_when_moon_home_unset() {
    let tmp = tempdir().expect("tempdir");
    let home = tmp.path().join("home");
    std::fs::create_dir_all(&home).expect("mkdir home");

    let expected = home.join("moon/state/moon_state.json");
    let legacy_nested = home.join("moon/moon/state/moon_state.json");

    let assert = assert_cmd::cargo::cargo_bin_cmd!("moon")
        .current_dir(tmp.path())
        .env("HOME", &home)
        .env_remove("MOON_HOME")
        .env_remove("MOON_STATE_FILE")
        .env_remove("MOON_STATE_DIR")
        .arg("status")
        .assert()
        .failure()
        .stdout(contains(format!("state_file={}", expected.display())));

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        !stdout.contains(&legacy_nested.display().to_string()),
        "status should not report legacy nested state path: {}",
        legacy_nested.display()
    );
}
