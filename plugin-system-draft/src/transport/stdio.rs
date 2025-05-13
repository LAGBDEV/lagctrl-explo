use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Stdin, Stdout};
use crate::transport::Transport;
use async_trait::async_trait;

/// A basic transport over STDIN/STDOUT.
/// It reads and writes raw `String` messages (one line per message).
pub struct StdioTransport {
    reader: BufReader<Stdin>,
    writer: Stdout,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self {
            reader: BufReader::new(io::stdin()),
            writer: io::stdout(),
        }
    }
}

#[async_trait]
impl Transport for StdioTransport {
    type Input = String;
    type Output = String;

    async fn read_message(&mut self) -> io::Result<Option<Self::Input>> {
        let mut line = String::new();
        let bytes_read = self.reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            // EOF
            Ok(None)
        } else {
            Ok(Some(line.trim_end().to_string()))
        }
    }

    async fn send_message(&mut self, msg: Self::Output) -> io::Result<()> {
        self.writer.write_all(msg.as_bytes()).await?;
        self.writer.write_all(b"\n").await?;
        self.writer.flush().await
    }
}
