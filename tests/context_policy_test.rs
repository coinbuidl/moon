use predicates::str::contains;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn write_fake_openclaw(bin_path: &Path, log_path: &Path, plugins_list_payload: &str) {
    let script = format!(
        r#"#!/usr/bin/env bash
echo "$@" >> "{}"
if [ "$1" = "plugins" ] && [ "$2" = "list" ]; then
  cat <<'JSON'
{}
JSON
fi
exit 0
"#,
        log_path.display(),
        plugins_list_payload
    );
    fs::write(bin_path, script).expect("write fake openclaw");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(bin_path).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(bin_path, perms).expect("chmod");
    }
}

fn write_context_policy(path: &Path) {
    fs::write(
        path,
        r#"[context]
window_mode = "inherit"
prune_mode = "disabled"
compaction_authority = "moon"
compaction_start_ratio = 0.78
compaction_emergency_ratio = 0.90
compaction_recover_ratio = 0.65
"#,
    )
    .expect("write moon context policy");
}

#[test]
fn install_applies_context_policy_and_status_reports_clean() {
    let tmp = tempdir().expect("tempdir");
    let state_dir = tmp.path().join("state");
    fs::create_dir_all(&state_dir).expect("mkdir");
    let config_path = state_dir.join("openclaw.json");
    fs::write(&config_path, "{}\n").expect("write config");
    let moon_config = tmp.path().join("moon.toml");
    write_context_policy(&moon_config);

    let fake_openclaw = tmp.path().join("openclaw");
    let log_path = tmp.path().join("openclaw.log");
    let plugins_list_payload = r#"{"plugins":[{"id":"moon","status":"loaded"}],"diagnostics":[]}"#;
    write_fake_openclaw(&fake_openclaw, &log_path, plugins_list_payload);

    assert_cmd::cargo::cargo_bin_cmd!("moon")
        .current_dir(tmp.path())
        .env("OPENCLAW_STATE_DIR", &state_dir)
        .env("OPENCLAW_CONFIG_PATH", &config_path)
        .env("OPENCLAW_BIN", &fake_openclaw)
        .env("MOON_CONFIG_PATH", &moon_config)
        .arg("install")
        .assert()
        .success();

    let cfg: Value = serde_json::from_str(&fs::read_to_string(&config_path).expect("read config"))
        .expect("parse cfg");
    assert!(
        cfg.get("agents")
            .and_then(|v| v.get("defaults"))
            .and_then(|v| v.get("contextTokens"))
            .is_none()
    );
    assert!(
        cfg.get("agents")
            .and_then(|v| v.get("defaults"))
            .and_then(|v| v.get("contextPruning"))
            .is_none()
    );
    assert_eq!(
        cfg.get("agents")
            .and_then(|v| v.get("defaults"))
            .and_then(|v| v.get("compaction"))
            .and_then(|v| v.get("mode"))
            .and_then(Value::as_str),
        Some("default")
    );

    assert_cmd::cargo::cargo_bin_cmd!("moon")
        .current_dir(tmp.path())
        .env("OPENCLAW_STATE_DIR", &state_dir)
        .env("OPENCLAW_CONFIG_PATH", &config_path)
        .env("OPENCLAW_BIN", &fake_openclaw)
        .env("MOON_CONFIG_PATH", &moon_config)
        .arg("status")
        .assert()
        .success();
}

#[test]
fn status_fails_when_compaction_mode_drift_reenables_openclaw_authority() {
    let tmp = tempdir().expect("tempdir");
    let state_dir = tmp.path().join("state");
    fs::create_dir_all(&state_dir).expect("mkdir");
    let config_path = state_dir.join("openclaw.json");
    fs::write(&config_path, "{}\n").expect("write config");
    let moon_config = tmp.path().join("moon.toml");
    write_context_policy(&moon_config);

    let fake_openclaw = tmp.path().join("openclaw");
    let log_path = tmp.path().join("openclaw.log");
    let plugins_list_payload = r#"{"plugins":[{"id":"moon","status":"loaded"}],"diagnostics":[]}"#;
    write_fake_openclaw(&fake_openclaw, &log_path, plugins_list_payload);

    assert_cmd::cargo::cargo_bin_cmd!("moon")
        .current_dir(tmp.path())
        .env("OPENCLAW_STATE_DIR", &state_dir)
        .env("OPENCLAW_CONFIG_PATH", &config_path)
        .env("OPENCLAW_BIN", &fake_openclaw)
        .env("MOON_CONFIG_PATH", &moon_config)
        .arg("install")
        .assert()
        .success();

    let mut cfg: Value =
        serde_json::from_str(&fs::read_to_string(&config_path).expect("read config"))
            .expect("parse cfg");
    cfg.get_mut("agents")
        .and_then(Value::as_object_mut)
        .and_then(|agents| agents.get_mut("defaults"))
        .and_then(Value::as_object_mut)
        .and_then(|defaults| defaults.get_mut("compaction"))
        .and_then(Value::as_object_mut)
        .expect("compaction object")
        .insert("mode".to_string(), Value::from("safeguard"));
    fs::write(
        &config_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&cfg).expect("serialize config")
        ),
    )
    .expect("write drifted config");

    assert_cmd::cargo::cargo_bin_cmd!("moon")
        .current_dir(tmp.path())
        .env("OPENCLAW_STATE_DIR", &state_dir)
        .env("OPENCLAW_CONFIG_PATH", &config_path)
        .env("OPENCLAW_BIN", &fake_openclaw)
        .env("MOON_CONFIG_PATH", &moon_config)
        .arg("status")
        .assert()
        .failure()
        .stdout(contains(
            "context policy drift: agents.defaults.compaction.mode expected default",
        ));
}
