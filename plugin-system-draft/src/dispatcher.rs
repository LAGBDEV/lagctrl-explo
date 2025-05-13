use crate::domain::{Command, Response};
use crate::errors::DispatcherError;
use crate::transport::Transport;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Handler: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn handle(&self, cmd: Command) -> Result<Response, Self::Error>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandKind {
    Process,
    Handshake,
    Shutdown,
}

pub struct Dispatcher<T, E>
where
    T: Transport<Input = Command, Output = Response> + Send,
    E: std::error::Error + Send + Sync + 'static,
{
    transport: T,
    handlers: HashMap<CommandKind, Box<dyn Handler<Error = DispatcherError<E>>>>,
}

impl<T, E> Dispatcher<T, E>
where
    T: Transport<Input = Command, Output = Response> + Send,
    E: std::error::Error + Send + Sync + 'static,
{
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            handlers: HashMap::new(),
        }
    }

    pub fn register<H>(&mut self, kind: CommandKind, handler: H)
    where
        H: Handler<Error = DispatcherError<E>> + 'static,
    {
        self.handlers.insert(kind, Box::new(handler));
    }

    pub async fn dispatch_loop(&mut self) -> Result<(), DispatcherError<E>> {
        while let Some(cmd) = self.transport.read_message().await? {
            let kind = get_command_kind(&cmd);

            let handler = match self.handlers.get(&kind) {
                Some(h) => h,
                None => return Err(DispatcherError::UnknownCommand(kind.clone())),
            };

            let response = handler.handle(cmd).await?;
            self.transport.send_message(response).await?;

            if kind == CommandKind::Shutdown {
                break;
            }
        }

        Ok(())
    }
}

fn get_command_kind(cmd: &Command) -> CommandKind {
    match cmd {
        Command::Process { .. } => CommandKind::Process,
        Command::Handshake { .. } => CommandKind::Handshake,
        Command::Shutdown => CommandKind::Shutdown,
    }
}
