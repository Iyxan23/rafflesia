use anyhow::{Result, Context};
use clap::{AppSettings, Command};

mod commands;

fn main() -> Result<()> {
    let args = Command::new("rafflesia")
        .about("A simple language for sketchware projects")
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand_required(true)
        .subcommands(commands::builtin())
        .get_matches();

    if let Some((subcommand, args)) = args.subcommand() {
        let func = commands::builtin_exec(subcommand)
            .with_context(format!("Subcommand {} does not exist", subcommand))?;

        func(args);
    }

    Ok(())
}
