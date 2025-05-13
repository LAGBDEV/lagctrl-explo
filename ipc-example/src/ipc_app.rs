use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let plugin_dir = "target/debug/";
    let plugin_name = "ipc-plugin";

    let mut plugin = Command::new(format!("{}{}", plugin_dir, plugin_name))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn plugin process");

    let mut stdin = plugin.stdin.take().expect("Failes to open stdin");
    let stdout = plugin.stdout.take().expect("Failes to open stdout");

    let mut reader = BufReader::new(stdout);

    writeln!(stdin, "ping")?;
    let mut line = String::new();
    reader.read_line(&mut line)?;
    println!("Plugin replied : {}", line);

    Ok(())
}
