use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AocError {
    Unsupported,
    Io(std::io::Error),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AocError::Unsupported => write!(f, "Día o año no soportado"),
            AocError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for AocError {}

impl From<std::io::Error> for AocError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
