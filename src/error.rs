use std::fmt;


#[derive(Debug, Clone)]
pub enum AppError {
    InvalidCommand(String),
    MissingArgument(String),
    RemoteCommandExecution(String),
    LocalCommandExecution(String, String),
    CannotAccessRunningDir,
    SessionChannel,
    Unknown,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            AppError::InvalidCommand(mode) => format!("{mode} is not a valid command. Use (new, init, configure, help)"),
            AppError::MissingArgument(arg) => format!("{arg} is required as via cli or config.yaml"),
            AppError::RemoteCommandExecution(cmd) => format!("Remote command \"{cmd}\" failed"),
            AppError::LocalCommandExecution(cmd, err) => format!("Local command \"{cmd}\" failed with details {err}"),
            AppError::CannotAccessRunningDir => "Failed to read path of current directory".into(),
            AppError::SessionChannel => "Error while accessing ssh session".into(),
            AppError::Unknown => "An unknown error occured".into(),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for AppError {}
