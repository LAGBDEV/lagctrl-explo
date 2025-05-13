use anyhow::Result;
use ipc_system::ipc_io::IpcIO;
use ipc_system::ipc_protocol::{IpcCommand, IpcResponse};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    // Spawn the child process
    let mut child = Command::new("target/debug/ipc-system-child")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    let child_stdin: tokio::process::ChildStdin = child.stdin.take().expect("no stdin");
    let child_stdout: tokio::process::ChildStdout = child.stdout.take().expect("no stdout");

    // Encapsulate reader+writer into our helper
    let mut ipc: IpcIO<tokio::process::ChildStdout, tokio::process::ChildStdin> =
        IpcIO::new(child_stdout, child_stdin);

    // Kick off writer in the background
    tokio::spawn(async move {
        let mut w = ipc.writer;

        // Handshake
        let hs = IpcCommand::Handshake { version: 1 }.to_line() + "\n";
        w.write_all(hs.as_bytes()).await.unwrap();

        // Process some data
        let proc = IpcCommand::Process {
            data: "rustacean".into(),
        }
        .to_line()
            + "\n";
        w.write_all(proc.as_bytes()).await.unwrap();

        // Tell child to shut down
        let bye = IpcCommand::Shutdown.to_line() + "\n";
        w.write_all(bye.as_bytes()).await.unwrap();
        w.shutdown().await.unwrap();
    });

    // Read lines as they come back
    while let Some(line) = ipc.lines.next_line().await? {
        if let Some(resp) = IpcResponse::from_line(&line) {
            match resp {
                IpcResponse::Greeting { message } => {
                    println!("Child says hello: {}", message);
                }
                IpcResponse::Processed { result } => {
                    println!("Processed result: {}", result);
                }
                IpcResponse::Error { error } => {
                    eprintln!("Error from child: {}", error);
                }
                IpcResponse::HandshakeAck { version } => println!("Child protocol v{}", version),
            }
        } else {
            eprintln!("Unrecognized line: {}", line);
        }
    }

    let status = child.wait().await?;
    println!("Child exited with: {}", status);
    Ok(())
}
