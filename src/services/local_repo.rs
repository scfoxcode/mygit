use std::process::Command;
use crate::error::AppError;
use crate::config::SSHConfig;

pub struct LocalRepo {}

impl LocalRepo {
    pub fn init() -> Result<(), AppError> {
        Command::new("git")
        .arg("init")
        .status()
        .map_err(|err| {
            return AppError::LocalCommandExecution("git init".into(), err.to_string())
        })?;
        Ok(())
    }

    pub fn new(repo_name: &str) -> Result<(), AppError> {
        Command::new("git")
        .arg("new")
        .arg(repo_name)
        .status()
        .map_err(|err| {
            return AppError::LocalCommandExecution("git new".into(), err.to_string())
        })?;
        Ok(())
    }

    //git remote add origin ssh://git@192.169.1.66/usr/local/git/<name>.git
    //git push --set-upstream origin master

    pub fn set_remote(repo_name: &str, cfg: &SSHConfig) -> Result<(), AppError> {
        let url = format!(
            "ssh://{}@{}:{}{}/{}.git",
            cfg.ssh_username,
            cfg.ssh_server,
            cfg.ssh_port,
            cfg.remote_repo_path,
            repo_name,
        );

        Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(url)
        .status()
        .map_err(|err| {
            return AppError::LocalCommandExecution("git remote add origin".into(), err.to_string())
        })?;


        Ok(())
    }
}

