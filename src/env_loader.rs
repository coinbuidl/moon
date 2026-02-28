use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DotenvLoadOutcome {
    LoadedDefault,
    LoadedFallback(PathBuf),
    Missing,
}

fn fallback_dotenv_path(moon_home: Option<PathBuf>, home_dir: Option<PathBuf>) -> Option<PathBuf> {
    let base = moon_home.or(home_dir)?;
    Some(base.join("moon/.env"))
}

pub fn load_dotenv() -> DotenvLoadOutcome {
    if dotenvy::dotenv().is_ok() {
        return DotenvLoadOutcome::LoadedDefault;
    }

    let fallback = fallback_dotenv_path(
        env::var_os("MOON_HOME").map(PathBuf::from),
        dirs::home_dir(),
    );

    let Some(path) = fallback else {
        return DotenvLoadOutcome::Missing;
    };
    if path.is_file() {
        if dotenvy::from_path(&path).is_ok() {
            return DotenvLoadOutcome::LoadedFallback(path);
        }
    }

    DotenvLoadOutcome::Missing
}

#[cfg(test)]
mod tests {
    use super::fallback_dotenv_path;
    use std::path::PathBuf;

    #[test]
    fn fallback_always_uses_moon_repo_subdir_for_moon_home() {
        let got = fallback_dotenv_path(
            Some(PathBuf::from("/workspace")),
            Some(PathBuf::from("/home/alice")),
        );

        let want = Some(PathBuf::from("/workspace/moon/.env"));
        assert_eq!(got, want);
    }

    #[test]
    fn fallback_uses_home_when_moon_home_unset() {
        let got = fallback_dotenv_path(None, Some(PathBuf::from("/home/alice")));
        let want = Some(PathBuf::from("/home/alice/moon/.env"));
        assert_eq!(got, want);
    }
}
