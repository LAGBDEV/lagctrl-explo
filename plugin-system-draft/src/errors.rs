#[derive(Debug)]
pub enum DispatcherError<E: std::error::Error + 'static> {
    Io(std::io::Error),
    Protocol(E),
    Handler(Box<dyn std::error::Error + Send + Sync>),
    UnknownCommand(crate::dispatcher::CommandKind),
}

use std::fmt;
use std::error::Error;

impl<E: Error + 'static> fmt::Display for DispatcherError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DispatcherError::Io(e) => write!(f, "IO error: {}", e),
            DispatcherError::Protocol(e) => write!(f, "Protocol error: {}", e),
            DispatcherError::Handler(e) => write!(f, "Handler error: {}", e),
            DispatcherError::UnknownCommand(cmd) => write!(f, "Unknown command: {:?}", cmd),
        }
    }
}

impl<E: Error + 'static> Error for DispatcherError<E> {}

impl<E: std::error::Error + 'static> From<std::io::Error> for DispatcherError<E> {
    fn from(e: std::io::Error) -> Self {
        DispatcherError::Io(e)
    }
}
