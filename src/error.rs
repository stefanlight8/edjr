//! Journal runtime errors
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    io,
};

#[derive(Debug)]
pub enum JournalError {
    OpenError(io::Error),
}

impl Error for JournalError {}

impl Display for JournalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            JournalError::OpenError(err) => write!(f, "failed to open journal: {:?}", err),
        }
    }
}

#[derive(Debug)]
pub enum JournalReadError {
    ReadError(io::Error),
    ParsingError(serde_json::Error),
}

impl Error for JournalReadError {}

impl Display for JournalReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            JournalReadError::ReadError(err) => write!(f, "failed to read journal: {:?}", err),
            JournalReadError::ParsingError(err) => write!(f, "failed to parse journal: {:?}", err),
        }
    }
}
