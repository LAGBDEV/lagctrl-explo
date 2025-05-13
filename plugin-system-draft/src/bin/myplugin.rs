use anyhow::Result;
use plugin_system_draft::{dispatcher::{CommandKind, Dispatcher}, handlers::ProcessHandler, protocol::json::JsonProtocol, protocol_transport::ProtocolTransport, transport::stdio::StdioTransport};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup raw stdio transport (reads and writes Strings)
    let raw = StdioTransport::new();

    // 2. Choose a protocol (e.g., JSON)
    let protocol = JsonProtocol;

    // 3. Wrap into ProtocolTransport which provides Command/Response transport
    let transport = ProtocolTransport::new(raw, protocol);

    // 4. Construct dispatcher with appropriate transport and error type
    let mut dispatcher: Dispatcher<_, _> = Dispatcher::new(transport);

    // 5. Register handlers for each command kind
    dispatcher.register(CommandKind::Process, ProcessHandler);

    // 6. Run main loop until Command::Shutdown is received
    dispatcher.dispatch_loop().await.map_err(|e| {
        eprintln!("Dispatcher error: {:?}", e);
        e
    })?;

    Ok(())
}
