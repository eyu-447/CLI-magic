#[derive(Debug)]
pub enum CliMagicError {
    InvalidInput(String),
    FileNotFound(String),
    Unknown(String),
}

impl std::fmt::Display for CliMagicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliMagicError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
            CliMagicError::FileNotFound(msg) => write!(f, "File Not Found: {}", msg),
            CliMagicError::Unknown(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}

impl std::error::Error for CliMagicError {}
