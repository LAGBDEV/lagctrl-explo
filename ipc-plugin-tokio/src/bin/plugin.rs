use ipc_plugin_tokio::run_plugin_loop;
use tokio::io::{self, stdin, stdout, BufReader};

#[tokio::main]
async fn main() -> io::Result<()> {
    eprintln!("Plugin PID = {}", std::process::id());
    let reader = BufReader::new(stdin());
    let writer = stdout();
    run_plugin_loop(reader, writer).await
}
