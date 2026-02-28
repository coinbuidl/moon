use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn mutating_commands_fail_outside_explicit_workspace() {
    let workspace = tempdir().expect("workspace tempdir");
    let run_dir = tempdir().expect("run tempdir");
    let moon_home = workspace.path().join("moon-home");

    assert_cmd::cargo::cargo_bin_cmd!("moon")
        .current_dir(run_dir.path())
        .env("MOON_HOME", &moon_home)
        .arg("moon-stop")
        .assert()
        .failure()
        .stderr(contains("E004_CWD_INVALID"));
}

#[test]
fn allow_out_of_bounds_bypasses_workspace_validation() {
    let workspace = tempdir().expect("workspace tempdir");
    let run_dir = tempdir().expect("run tempdir");
    let moon_home = workspace.path().join("moon-home");

    assert_cmd::cargo::cargo_bin_cmd!("moon")
        .current_dir(run_dir.path())
        .env("MOON_HOME", &moon_home)
        .args(["--allow-out-of-bounds", "moon-stop"])
        .assert()
        .success();
}
