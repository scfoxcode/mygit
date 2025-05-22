use clap::Args;
use crate::error::AppError;

use super::CommandRunner;

#[derive(Args, Debug)]
pub struct ConfigureCommand {}

impl CommandRunner for ConfigureCommand {
    fn execute(&self) -> Result<(), AppError> {
        println!("TODO implement ConfigureCommand");
        Ok(())
    }
}
