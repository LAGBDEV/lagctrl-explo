use capnp::message::{Builder, ReaderOptions};
use capnp_futures::serialize_packed;
use std::io;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

#[allow(dead_code)]
pub mod protocol_capnp {
    include!(concat!(env!("OUT_DIR"), "/protocol_capnp.rs"));
}

/// Fully async loop over packed Cap’n Proto messages.
pub async fn run_plugin_loop<R, W>(reader: R, writer: W) -> io::Result<()>
where
    R: tokio::io::AsyncRead + Unpin,
    W: tokio::io::AsyncWrite + Unpin,
{
    // wrap both ends in the compat adapters
    let mut reader = reader.compat(); // now impl futures_io::AsyncRead + Unpin
    let mut writer = writer.compat_write(); // now impl futures_io::AsyncWrite + Unpin

    loop {
        // -------- Async read one Cap’n Proto message --------
        let message_reader = serialize_packed::read_message(&mut reader, ReaderOptions::new())
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let cmd = message_reader
            .get_root::<protocol_capnp::command::Reader>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // -------- Build the response --------
        let mut response_msg = Builder::new_default();
        {
            let mut resp = response_msg.init_root::<protocol_capnp::response::Builder>();
            match cmd.which().unwrap() {
                protocol_capnp::command::Which::Ping(()) => {
                    resp.set_pong("pong");
                }
                protocol_capnp::command::Which::Echo(text) => {
                    resp.set_text(text.unwrap_or("".into()));
                }
                protocol_capnp::command::Which::Add(add) => {
                    resp.set_number(add.as_ref().unwrap().get_a() + add.unwrap().get_b());
                }
                protocol_capnp::command::Which::Time(()) => {
                    let now = chrono::Utc::now().to_rfc3339();
                    resp.set_text(&now);
                }
                protocol_capnp::command::Which::GetInitialUI(()) => {
                    let tree = resp.init_ui_init();
                    tree.init_ui_components(0);
                }
            }
        }

        // -------- Async write it back, packed --------
        serialize_packed::write_message(&mut writer, &response_msg)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
}
