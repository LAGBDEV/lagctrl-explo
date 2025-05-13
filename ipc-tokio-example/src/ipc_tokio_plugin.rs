use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> io::Result<()> {
    // (Optional) print PID so you can attach a debugger
    eprintln!("Plugin PID = {}", std::process::id());

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut lines = BufReader::new(stdin).lines();
    let mut out = stdout;

    while let Some(line) = lines.next_line().await? {
        if line.trim() == "ping" {
            out.write_all(b"pong\n").await?;
            out.flush().await?;
        }
    }

    Ok(())
}
