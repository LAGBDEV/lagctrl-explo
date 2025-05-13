use ipc_plugin_capnp::run_plugin_loop;
use tokio::io::{stdin, stdout, BufReader};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    eprintln!("Plugin PID = {}", std::process::id());
    let reader = BufReader::new(stdin());
    let writer = stdout();
    run_plugin_loop(reader, writer).await
}
