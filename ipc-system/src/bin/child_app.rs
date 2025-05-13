use anyhow::Result;
use ipc_system::ipc_io::IpcIO;
use ipc_system::ipc_protocol::{IpcCommand, IpcResponse};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Plugin PID = {}", std::process::id());

    // Wrap STDIN/STDOUT
    let ipc: IpcIO<tokio::io::Stdin, tokio::io::Stdout> =
        IpcIO::new(tokio::io::stdin(), tokio::io::stdout());
    let mut lines = ipc.lines;
    let mut w = ipc.writer;

    // Loop over incoming commands
    while let Some(line) = lines.next_line().await? {
        let response = match IpcCommand::from_line(&line) {
            Some(IpcCommand::Process { data }) => {
                // Example processing: reverse the string
                let result = data.chars().rev().collect::<String>();
                IpcResponse::Processed { result }
            }
            Some(IpcCommand::Handshake { version }) => IpcResponse::Processed {
                result: version.to_string(),
            },
            Some(IpcCommand::Shutdown) => break,
            None => IpcResponse::Error {
                error: format!("Unknown command: {}", line),
            },
        };

        // Send the response
        let out = response.to_line() + "\n";
        w.write_all(out.as_bytes()).await?;
    }

    w.flush().await?;
    Ok(())
}
