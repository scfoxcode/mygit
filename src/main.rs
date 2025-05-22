use clap::Parser;
use crate::command::CommandRunner;

mod command;
mod error;
mod config;

fn main() -> Result<(), anyhow::Error> {

    let args = config::Cli::parse();

    args.command.execute();


    Ok(())
}
