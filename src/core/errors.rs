use std::fmt;

#[derive(Debug)]
pub enum MyWayError {
    ProjectNotFound(String),
    VersionAlreadyExists(String),
    ProjectAlreadyExists(String),
    InvalidInput(String),
    StringLengthLimitExceeded(String),
    IoError(std::io::Error),
    StacksIsEmpty(String),
    WayLengthExceeded(String)
}

impl fmt::Display for MyWayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyWayError::IoError(e) => write!(f, "IO Error: {}", e),
            MyWayError::ProjectNotFound(id) => write!(f, "Project not found: {}", id),
            MyWayError::InvalidInput(e) => write!(f, "Invalid Input: {}", e),
            MyWayError::ProjectAlreadyExists(e) => write!(f, "Project already exist: {}", e),
            MyWayError::VersionAlreadyExists(e) => write!(f, "Version already exist: {}", e),
            MyWayError::StacksIsEmpty(e) => write!(f, "Stack list is empty: {}", e),
            MyWayError::WayLengthExceeded(e) => write!(f, "Stack list length exceeded: {}", e),
            MyWayError::StringLengthLimitExceeded(e) => write!(f, "String length limit exceeded: {}", e),
        }
    }
}

impl std::error::Error for MyWayError {}