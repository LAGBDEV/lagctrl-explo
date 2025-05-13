use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};

/// Runs the plugin loop: reads JSON lines from `reader`, processes via `protocol`, writes JSON to `writer`.
pub async fn run_plugin_loop<R, W>(reader: R, mut writer: W) -> std::io::Result<()>
where
    R: AsyncBufRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        // Parse and execute
        let response = match serde_json::from_str::<protocol::Command>(&line) {
            Ok(cmd) => cmd.execute(),
            Err(e) => protocol::Response::Text(format!("error: {}", e)),
        };

        // Serialize and send
        let json =
            serde_json::to_string(&response).unwrap_or_else(|e| format!("{{\"error\":\"{}\"}}", e));
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
    }
    Ok(())
}

// Expose protocol module
pub mod protocol;
