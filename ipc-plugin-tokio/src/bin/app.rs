use anyhow::Result;
use ipc_plugin_tokio::protocol::Command as PluginCommand;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    // Spawn the plugin process
    let plugin_dir = "target/debug/";
    let plugin_name = "ipc-plugin-tokio-plugin";

    let mut child = Command::new(format!("{}{}", plugin_dir, plugin_name))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn plugin");

    // Take ownership of its stdin and stdout
    let mut plugin_stdin = child.stdin.take().expect("Failed to open stdin");
    let plugin_stdout = child.stdout.take().expect("Failed to open stdout");
    let mut lines = BufReader::new(plugin_stdout).lines();

    // Example: send a Ping command
    let ping_cmd = serde_json::to_string(&PluginCommand::GetInitialUI())? + "\n";
    plugin_stdin.write_all(ping_cmd.as_bytes()).await?;
    plugin_stdin.flush().await?;

    // Close the stdin pipe to signal EOF to the plugin
    drop(plugin_stdin);

    // Read and print the response
    if let Some(response) = lines.next_line().await? {
        println!("Plugin response: {}", response);
    }

    // Optionally wait for the plugin to exit
    let status = child.wait().await?;
    println!("Plugin exited with: {}", status);

    Ok(())
}
