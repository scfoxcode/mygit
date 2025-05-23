use std::io::Read;
use crate::config::SSHConfig;
use crate::error::AppError;
use super::Ssh;

pub struct RemoteRepo {}

impl RemoteRepo {
    pub fn create(repo_name: &str, cfg: &SSHConfig) -> Result<(), AppError> {

        let mut channel = Ssh::open_channel(cfg)?;

        let command = format!("cd /usr/local/git && git init --bare {}.git", repo_name);

        channel.exec(&command).map_err(|_| AppError::RemoteCommandExecution(command))?;

        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        println!("Output: {}", output);


        channel.wait_close().unwrap();
        println!("Exit Status: {}", channel.exit_status().unwrap());

        Ok(())
    }
}

