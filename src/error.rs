use std::{error, fmt, io};

#[derive(Debug)]
pub enum AppError {
    IO(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occurred")
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl error::Error for AppError {}
