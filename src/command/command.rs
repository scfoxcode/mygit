use crate::error::AppError;

pub trait CommandRunner {
    fn execute(&self) -> Result<(), AppError>;
}
