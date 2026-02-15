use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::commands;

#[derive(Debug, Parser)]
#[command(name = "oc-token-optim")]
#[command(about = "OpenClaw context optimization installer/repair CLI")]
pub struct Cli {
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Install(InstallArgs),
    Verify(VerifyArgs),
    Repair(RepairArgs),
    PostUpgrade,
    Status,
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[arg(long)]
    pub force: bool,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub apply: bool,
}

#[derive(Debug, Args, Default)]
pub struct VerifyArgs {
    #[arg(long)]
    pub strict: bool,
}

#[derive(Debug, Args, Default)]
pub struct RepairArgs {
    #[arg(long)]
    pub force: bool,
}

fn print_report(report: &commands::CommandReport, as_json: bool) -> Result<()> {
    if as_json {
        println!("{}", serde_json::to_string_pretty(report)?);
        return Ok(());
    }

    println!("command: {}", report.command);
    println!("ok: {}", report.ok);
    if !report.details.is_empty() {
        println!("details:");
        for detail in &report.details {
            println!("- {detail}");
        }
    }
    if !report.issues.is_empty() {
        println!("issues:");
        for issue in &report.issues {
            println!("- {issue}");
        }
    }
    Ok(())
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let report = match &cli.command {
        Command::Install(args) => commands::install::run(&commands::install::InstallOptions {
            force: args.force,
            dry_run: args.dry_run,
            apply: args.apply,
        })?,
        Command::Verify(args) => commands::verify::run(&commands::verify::VerifyOptions {
            strict: args.strict,
        })?,
        Command::Repair(args) => {
            commands::repair::run(&commands::repair::RepairOptions { force: args.force })?
        }
        Command::PostUpgrade => commands::post_upgrade::run()?,
        Command::Status => commands::status::run()?,
    };

    print_report(&report, cli.json)?;

    if report.ok {
        Ok(())
    } else {
        std::process::exit(2);
    }
}
