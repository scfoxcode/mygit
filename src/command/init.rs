use clap::Args;
use std::io::BufReader;
use std::fs::File;
use dirs::home_dir;
use crate::error::AppError;
use crate::config::{PartialConfig, SSHConfig};

use super::CommandRunner;

#[derive(Args, Debug)]
pub struct InitCommand {
    #[arg(short = 'k', long = "key-path")]
    pub key_path: Option<String>,

    #[arg(short = 'u', long = "user")]
    pub ssh_username: Option<String>,

    #[arg(short = 's', long = "server")]
    pub ssh_server: Option<String>,

    #[arg(short = 'p', long = "port")]
    pub ssh_port: Option<String>,

    #[arg(short = 'r', long = "remote-path")]
    pub remote_repo_path: Option<String>,
}

impl InitCommand {
    pub fn build_ssh_config(&self) -> Result<SSHConfig, AppError> {
        let path = home_dir().unwrap().join(".config/mygit/config.yaml");
        let file = File::open(path).unwrap(); // dangerous
        let reader = BufReader::new(file); 
        let partial: PartialConfig = serde_yaml::from_reader(reader).unwrap();

        Ok(
            SSHConfig {
                key_path:
                    self.key_path.clone()
                    .or(partial.key_path)
                    .ok_or(AppError::MissingArgument("key_path".into()))?,
                ssh_username:
                    self.ssh_username.clone()
                    .or(partial.ssh_username)
                    .ok_or(AppError::MissingArgument("ssh_username".into()))?,
                ssh_server:
                    self.ssh_server.clone()
                    .or(partial.ssh_server)
                    .ok_or(AppError::MissingArgument("ssh_server".into()))?,
                ssh_port:
                    self.ssh_port.clone()
                    .or(partial.ssh_port)
                    .ok_or(AppError::MissingArgument("ssh_port".into()))?,
                remote_repo_path:
                    self.remote_repo_path.clone()
                    .or(partial.remote_repo_path)
                    .ok_or(AppError::MissingArgument("remote_repo_path".into()))?,
            }
        )
    }
}

impl CommandRunner for InitCommand {
    fn execute(&self) -> Result<(), AppError> {
        let _cfg = self.build_ssh_config()?;
        println!("TODO implement InitCommand");
        Ok(())
    }
}
