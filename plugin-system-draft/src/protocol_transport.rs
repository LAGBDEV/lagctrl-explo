use async_trait::async_trait;
use std::io;
use crate::protocol::Protocol;
use crate::domain::{Command, Response};
use crate::transport::Transport;

pub struct ProtocolTransport<T, P> {
    inner: T,
    protocol: P,
}

impl<T, P> ProtocolTransport<T, P> {
    pub fn new(inner: T, protocol: P) -> Self {
        Self { inner, protocol }
    }
}

#[async_trait]
impl<T, P> Transport for ProtocolTransport<T, P>
where
    T: Transport<Input = String, Output = String> + Send,
    P: Protocol + Send + Sync + std::fmt::Debug,
{
    type Input = Command;
    type Output = Response;

    async fn read_message(&mut self) -> io::Result<Option<Self::Input>> {
        if let Some(line) = self.inner.read_message().await? {
            self.protocol
                .decode(&line)
                .map(Some)
                .map_err(|e: <P as Protocol>::Error| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))
        } else {
            Ok(None)
        }
    }

    async fn send_message(&mut self, msg: Self::Output) -> io::Result<()> {
        let raw = self.protocol.encode(&msg);
        self.inner.send_message(raw).await
    }
}
