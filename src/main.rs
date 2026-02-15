mod assets;
mod cli;
mod commands;
mod error;
mod logging;
mod openclaw;

fn main() {
    if let Err(err) = cli::run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}
