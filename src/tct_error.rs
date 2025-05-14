//! # === tct_error ===
//!
//! Defines the different errors the program
//! is able to throw. used to locate breaking
//! functions faster and handle different cases.

use std::fmt;
use std::io;

/// thrown by console_io module
#[derive(Debug)]
pub enum IoError {
    FailedFlush,
//     FailedWrite,
    FailedRead,
    Io(io::Error),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::FailedFlush => write!(f, "failed to flush"),
//             IoError::FailedWrite => write!(f, "failed to write"),
            IoError::FailedRead => write!(f, "failed to read"),
            IoError::Io(e) => write!(f, "got std::io::Error: {}", e),
        }
    }
}

impl std::error::Error for IoError {}

/// thrown by file_io
#[derive(Debug)]
pub enum FileError {
    PermissionDenied,
    FailedRead,
    FailedWrite,
    InvalidDirectory,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::PermissionDenied => write!(f, "permisson denied!"),
            FileError::FailedRead => write!(f, "failed to read"),
            FileError::FailedWrite => write!(f, "failed to write"),
            FileError::InvalidDirectory => write!(f, "invalid directory"),
        }
    }
}

impl std::error::Error for FileError {}

/// currently unused
#[derive(Debug)]
pub enum BufferError {
    BufferOverflow,
    BufferUnderflow,
    BufferNotFound
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferError::BufferOverflow => write!(f, "overflow occured in buffer"),
            BufferError::BufferUnderflow => write!(f, "underflow occured in buffer"),
            BufferError::BufferNotFound => write!(f, "buffer could not be found"),
        }
    }
}

impl std::error::Error for BufferError {}

/// general Errors
#[derive(Debug)]
pub enum TctError {
    TimeNotFound
}

impl fmt::Display for TctError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TctError::TimeNotFound => write!(f, "failed loading local time.")
        }
    }
}
