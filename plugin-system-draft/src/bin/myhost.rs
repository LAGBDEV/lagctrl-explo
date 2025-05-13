use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::process::{Command, ChildStdin, ChildStdout};
use anyhow::Result;
use serde_json;
use plugin_system_draft::domain::{Command as PluginCommand, Response as PluginResponse};

#[tokio::main]
async fn main() -> Result<()> {
    // Launch the plugin binary
    let mut child = Command::new("target/debug/myplugin")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let stdin: ChildStdin = child.stdin.take().expect("Failed to open plugin stdin");
    let stdout: ChildStdout = child.stdout.take().expect("Failed to open plugin stdout");

    let mut writer = tokio::io::BufWriter::new(stdin);
    let mut reader = BufReader::new(stdout).lines();

    // Send a command
    let command = PluginCommand::Process { data: "Hello plugin".into() };
    let wire = serde_json::to_string(&command)? + "\n";
    writer.write_all(wire.as_bytes()).await?;
    writer.flush().await?;

    // Read a response
    if let Some(line) = reader.next_line().await? {
        let response: PluginResponse = serde_json::from_str(&line)?;
        println!("Received from plugin: {:?}", response);
    }

    Ok(())
}
