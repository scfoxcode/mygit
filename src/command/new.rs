use std::io::BufReader;
use std::fs::File;
use clap::Args;
use dirs::home_dir;
use crate::error::AppError;
use crate::config::{PartialConfig, SSHConfig};
use crate::services::RemoteRepo;
use crate::services::LocalRepo;
use crate::merge_configs;

use super::CommandRunner;

#[derive(Args, Debug)]
pub struct NewCommand {
    pub repo_name: String,

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

impl NewCommand {
    pub fn build_ssh_config(&self) -> Result<SSHConfig, AppError> {
        let path = home_dir().unwrap().join(".config/mygit/config.yaml");
        let file = File::open(path).unwrap(); // dangerous
        let reader = BufReader::new(file); 
        let partial: PartialConfig = serde_yaml::from_reader(reader).unwrap();

        Ok(
            SSHConfig {
                key_path: merge_configs!(self, partial, key_path)?,
                ssh_username: merge_configs!(self, partial, ssh_username)?,
                ssh_server: merge_configs!(self, partial, ssh_server)?,
                ssh_port: merge_configs!(self, partial, ssh_port)?,
                remote_repo_path: merge_configs!(self, partial, remote_repo_path)?,
            }
        )
    }
}

impl CommandRunner for NewCommand {
    fn execute(&self) -> Result<(), AppError> {
        let cfg = self.build_ssh_config()?;

        RemoteRepo::create(&self.repo_name, &cfg)?;
        LocalRepo::new(&self.repo_name)?; // TODO should we delete remote if this fails?
        LocalRepo::set_remote(&self.repo_name, &cfg)?;

        Ok(())
    }
}
