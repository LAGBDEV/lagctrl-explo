use async_trait::async_trait;
use crate::domain::{Command, Response};
use crate::dispatcher::{Handler};
use crate::errors::DispatcherError;

/// Handles `Command::Process` by reversing the input string.
pub struct ProcessHandler;

#[async_trait]
impl Handler for ProcessHandler {
    type Error = DispatcherError<std::io::Error>;

    async fn handle(&self, cmd: Command) -> Result<Response, Self::Error> {
        match cmd {
            Command::Process { data } => {
                let result = data.chars().rev().collect::<String>();
                Ok(Response::Processed { result })
            }
            _ => Err(DispatcherError::Handler(Box::new(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("ProcessHandler received unsupported command: {:?}", cmd),
                ),
            ))),
        }
    }
}
