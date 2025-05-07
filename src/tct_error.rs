use std::fmt;
use std::io;

#[derive(Debug)]
pub enum IoError {
    FailedFlush,
    FailedWrite,
    FailedRead,
    Io(io::Error),
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::FailedFlush => write!(f, "failed to flush"),
            IoError::FailedWrite => write!(f, "failed to write"),
            IoError::FailedRead => write!(f, "failed to read"),
            IoError::Io(e) => write!(f, "got std::io::Error: {}", e),
        }
    }
}

impl std::error::Error for IoError {}

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

#[derive(Debug)]
pub enum BufferError {
    BufferOverflow,
    BufferUnderflow,
    BufferNotFound
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferError::BufferOverflow => write!(f, ""),
            BufferError::BufferUnderflow => write!(f, ""),
            BufferError::BufferNotFound => write!(f, ""),
        }
    }
}
