use std::{error::Error, fmt::Display, io};

pub type EmbargoResult = Result<Option<String>, EmbargoError>;

/// Embargo should always fail gracefully. Errors should simply be reported to the console before termination of the program; thus the only error behavior we are concerned about is the error message
#[derive(Clone, Debug)]
pub struct EmbargoError {
    message: String,
}

impl EmbargoError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl Display for EmbargoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for EmbargoError {}

impl From<Box<dyn Error>> for EmbargoError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::new(&value.to_string())
    }
}

impl From<io::Error> for EmbargoError {
    fn from(value: io::Error) -> Self {
        Self::new(&value.to_string())
    }
}

impl From<toml::de::Error> for EmbargoError {
    fn from(value: toml::de::Error) -> Self {
        Self::new(value.message())
    }
}

impl From<toml::ser::Error> for EmbargoError {
    fn from(value: toml::ser::Error) -> Self {
        Self::new(&value.to_string())
    }
}

impl From<walkdir::Error> for EmbargoError {
    fn from(value: walkdir::Error) -> Self {
        Self::new(&value.to_string())
    }
}