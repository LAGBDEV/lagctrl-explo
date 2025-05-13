use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, Lines};

/// A helper bundling a line-based reader and a writer for IPC.
pub struct IpcIO<R, W> {
    pub lines: Lines<BufReader<R>>,
    pub writer: W,
}

impl<R, W> IpcIO<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    /// Wraps any AsyncRead + AsyncWrite into a buffered line reader and writer.
    pub fn new(read: R, write: W) -> Self {
        let reader = BufReader::new(read).lines();
        IpcIO {
            lines: reader,
            writer: write,
        }
    }

    /// Example behavior: read all lines, uppercase them, and write back.
    pub async fn echo_uppercase(mut self) -> Result<()> {
        while let Some(line) = self.lines.next_line().await? {
            let out = format!("CHILD RECEIVED: {}\n", line.to_uppercase());
            self.writer.write_all(out.as_bytes()).await?;
        }
        self.writer.flush().await?;
        Ok(())
    }
}
