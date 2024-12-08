use std::{error::Error, fmt::Display};

pub type EmbargoResult = Result<String, EmbargoError>;

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