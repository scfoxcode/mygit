use std::fmt;


#[derive(Debug, Clone)]
pub enum AppError {
    InvalidCommand(String),
    MissingArgument(String),
    Unknown,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            AppError::InvalidCommand(mode) => format!("{mode} is not a valid command. Use (new, init, configure, help)"),
            AppError::MissingArgument(arg) => format!("{arg} is required as via cli or config.yaml"),
            AppError::Unknown => "An unknown error occured".into(),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for AppError {}
