use anyhow::Result;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MoonPaths {
    pub moon_home: PathBuf,
    pub archives_dir: PathBuf,
    pub memory_dir: PathBuf,
    pub memory_file: PathBuf,
    pub logs_dir: PathBuf,
    pub openclaw_sessions_dir: PathBuf,
    pub qmd_bin: PathBuf,
    pub qmd_db: PathBuf,
    pub moon_home_is_explicit: bool,
}

fn required_home_dir() -> Result<PathBuf> {
    if let Some(home) = dirs::home_dir() {
        return Ok(home);
    }
    Err(anyhow::anyhow!("HOME directory could not be resolved"))
}

fn env_or_default_path(var: &str, fallback: PathBuf) -> PathBuf {
    match env::var(var) {
        Ok(v) if !v.trim().is_empty() => PathBuf::from(v.trim()),
        _ => fallback,
    }
}

fn moon_home_from_inputs(home: PathBuf, moon_home_env: Option<&str>) -> (PathBuf, bool) {
    match moon_home_env {
        Some(v) if !v.trim().is_empty() => (PathBuf::from(v.trim()), true),
        _ => (home, false),
    }
}

pub fn resolve_paths() -> Result<MoonPaths> {
    let home = required_home_dir()?;
    let moon_home_env = env::var("MOON_HOME").ok();
    let (moon_home, is_explicit) = moon_home_from_inputs(home.clone(), moon_home_env.as_deref());

    let archives_dir = env_or_default_path("MOON_ARCHIVES_DIR", moon_home.join("archives"));
    let memory_dir = env_or_default_path("MOON_MEMORY_DIR", moon_home.join("memory"));
    let memory_file = env_or_default_path("MOON_MEMORY_FILE", moon_home.join("MEMORY.md"));
    let logs_dir = env_or_default_path("MOON_LOGS_DIR", moon_home.join("moon/logs"));
    let openclaw_sessions_dir = env_or_default_path(
        "OPENCLAW_SESSIONS_DIR",
        home.join(".openclaw/agents/main/sessions"),
    );
    let qmd_bin = env_or_default_path("QMD_BIN", home.join(".bun/bin/qmd"));
    let qmd_db = env_or_default_path("QMD_DB", home.join(".cache/qmd/index.sqlite"));

    Ok(MoonPaths {
        moon_home,
        archives_dir,
        memory_dir,
        memory_file,
        logs_dir,
        openclaw_sessions_dir,
        qmd_bin,
        qmd_db,
        moon_home_is_explicit: is_explicit,
    })
}

#[cfg(test)]
mod tests {
    use super::moon_home_from_inputs;
    use std::path::PathBuf;

    #[test]
    fn default_moon_home_uses_home_root_when_unset() {
        let home = PathBuf::from("/home/alice");
        let (moon_home, is_explicit) = moon_home_from_inputs(home.clone(), None);
        assert_eq!(moon_home, home);
        assert!(!is_explicit);
    }

    #[test]
    fn explicit_moon_home_is_preserved() {
        let (moon_home, is_explicit) =
            moon_home_from_inputs(PathBuf::from("/home/alice"), Some("/workspace"));
        assert_eq!(moon_home, PathBuf::from("/workspace"));
        assert!(is_explicit);
    }

    #[test]
    fn blank_moon_home_falls_back_to_home_root() {
        let home = PathBuf::from("/home/alice");
        let (moon_home, is_explicit) = moon_home_from_inputs(home.clone(), Some("   "));
        assert_eq!(moon_home, home);
        assert!(!is_explicit);
    }
}
