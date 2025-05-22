use clap::{Parser, Subcommand};
use serde::Deserialize;
use crate::command::{CommandRunner, NewCommand, InitCommand, ConfigureCommand};
use crate::error::AppError;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(InitCommand), 
    New(NewCommand),
    Configure(ConfigureCommand), // starts interactive input
}

// This is a bit annoying considering all variants must implement it...
// It's because we want the enum to handle sub commands, but idealy would use the trait
// to call execute without caring about the type
// Suggests strongly that the structure needs improving
impl CommandRunner for Command{
    fn execute(&self) -> Result<(), AppError> {
        match self {
            Command::Init(cmd) => cmd.execute(),
            Command::New(cmd) => cmd.execute(),
            Command::Configure(cmd) => cmd.execute(),

        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "mygit", version, about = "A Git repo utility over SSH")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Deserialize)]
pub struct PartialConfig {
    pub key_path: Option<String>,
    pub ssh_username: Option<String>,
    pub ssh_server: Option<String>,
    pub ssh_port: Option<String>,
    pub remote_repo_path: Option<String>,
}

pub struct SSHConfig {
    pub key_path: String,
    pub ssh_username: String,
    pub ssh_server: String,
    pub ssh_port: String,
    pub remote_repo_path: String,
}
