mod assets;
mod cli;
mod commands;
mod env_loader;
mod error;
mod logging;
mod moon;
mod openclaw;

fn main() {
    if matches!(
        env_loader::load_dotenv(),
        env_loader::DotenvLoadOutcome::Missing
    ) {
        eprintln!("WARN: `.env` not found — distill/embed features will be unavailable.");
    }

    if let Err(err) = cli::run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}
