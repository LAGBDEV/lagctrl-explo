use anyhow::Result;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let plugin_dir = "target/debug/";
    let plugin_name = "ipc-tokio-plugin";

    //let mut plugin = Command::new(format!("{}{}", plugin_dir, plugin_name))
    // Spawn the plugin process, unwrap on error
    let mut child = Command::new(format!("{}{}", plugin_dir, plugin_name))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn plugin");

    // Take ownership of its stdin/stdout
    let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");
    let mut lines = BufReader::new(child_stdout).lines();

    // Send "ping\n"
    child_stdin.write_all(b"ping\n").await?;
    child_stdin.flush().await?;

    // Read back a line
    if let Some(reply) = lines.next_line().await? {
        println!("Host got: {}", reply);
    }

    // Wait for the child to exit
    let status = child.wait().await?;
    println!("Plugin exited: {}", status);
    Ok(())
}
