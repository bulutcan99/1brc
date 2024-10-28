use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ThreadPoolError {
    ShutdownTimeout,
    ThreadJoinError(String),
}

impl fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadPoolError::ShutdownTimeout => write!(f, "ThreadPool shutdown timed out"),
            ThreadPoolError::ThreadJoinError(id) => {
                write!(f, "Failed to join worker thread with ID: {}", id)
            }
        }
    }
}

impl Error for ThreadPoolError {}
