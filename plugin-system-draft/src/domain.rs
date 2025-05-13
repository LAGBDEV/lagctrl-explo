use serde::{Deserialize, Serialize};

/// All the commands your plugin host understands, regardless of wire format.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Command {
    Process { data: String },
    Handshake { version: String },
    Shutdown,
}

/// All the responses your plugin sends back.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    ShutdownAck,
    Processed { result: String },
    Error { error: String },
}

// Define HandlerError if it doesn't exist in the domain module
#[derive(Debug)]
pub enum HandlerError {
    InvalidCommand,
}
