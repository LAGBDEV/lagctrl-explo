// ipc_protocol.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcCommand {
    /// Parent initiates a handshake, sending its protocol version
    Handshake { version: u32 },
    /// Parent asks child to process some data
    Process { data: String },
    /// Parent asks child to shut down
    Shutdown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcResponse {
    /// Child greets parent
    Greeting { message: String },
    /// Child acknowledges handshake with its protocol version
    HandshakeAck { version: u32 },
    /// Child’s processed result
    Processed { result: String },
    /// Child reports an error
    Error { error: String },
}

impl IpcCommand {
    /// Format a command as a single line, e.g.:
    /// - `Hello`
    /// - `Process some data here`
    /// - `Shutdown`
    pub fn to_line(&self) -> String {
        match self {
            IpcCommand::Shutdown => "Shutdown".into(),
            IpcCommand::Process { data } => format!("Process {}", data),
            IpcCommand::Handshake { version } => format!("Handshake {}", version),
        }
    }

    /// Try to parse a line into a command.
    /// Returns `None` if it doesn’t match any variant.
    pub fn from_line(line: &str) -> Option<Self> {
        let mut parts = line.splitn(2, ' ');
        match parts.next()? {
            "Shutdown" => Some(IpcCommand::Shutdown),
            "Process" => {
                let data = parts.next().unwrap_or_default().to_string();
                Some(IpcCommand::Process { data })
            }
            "Handshake" => parts
                .next()
                .and_then(|s| s.parse().ok())
                .map(|v| IpcCommand::Handshake { version: v }),
            _ => None,
        }
    }
}

impl IpcResponse {
    /// Format a response as a single line, e.g.:
    /// - `Greeting Hi there`
    /// - `Processed RESULT_TEXT`
    /// - `Error Something went wrong`
    pub fn to_line(&self) -> String {
        match self {
            IpcResponse::Greeting { message } => format!("Greeting {}", message),
            IpcResponse::Processed { result } => format!("Processed {}", result),
            IpcResponse::Error { error } => format!("Error {}", error),
            IpcResponse::HandshakeAck { version } => format!("HandshakeAck {}", version),
        }
    }

    /// Try to parse a line into a response.
    /// Returns `None` if it doesn’t match any variant.
    pub fn from_line(line: &str) -> Option<Self> {
        let mut parts = line.splitn(2, ' ');
        match parts.next()? {
            "Greeting" => {
                let message = parts.next().unwrap_or_default().to_string();
                Some(IpcResponse::Greeting { message })
            }
            "Processed" => {
                let result = parts.next().unwrap_or_default().to_string();
                Some(IpcResponse::Processed { result })
            }
            "Error" => {
                let error = parts.next().unwrap_or_default().to_string();
                Some(IpcResponse::Error { error })
            }
            "HandshakeAck" => parts
                .next()
                .and_then(|s| s.parse().ok())
                .map(|v| IpcResponse::HandshakeAck { version: v }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_commands() {
        let cmds = vec![
            IpcCommand::Process {
                data: "foo bar".into(),
            },
            IpcCommand::Shutdown,
        ];
        for cmd in cmds {
            let line = cmd.to_line();
            let parsed = IpcCommand::from_line(&line).unwrap();
            assert_eq!(parsed, cmd);
        }
    }

    #[test]
    fn roundtrip_responses() {
        let resps = vec![
            IpcResponse::Greeting {
                message: "hey".into(),
            },
            IpcResponse::Processed {
                result: "OK".into(),
            },
            IpcResponse::Error {
                error: "oops".into(),
            },
        ];
        for resp in resps {
            let line = resp.to_line();
            let parsed = IpcResponse::from_line(&line).unwrap();
            assert_eq!(parsed, resp);
        }
    }

    #[test]
    fn invalid_lines() {
        assert!(IpcCommand::from_line("Unknown stuff").is_none());
        assert!(IpcResponse::from_line("Foo bar").is_none());
    }
}
