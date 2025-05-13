use async_trait::async_trait;
use std::io;

#[async_trait]
pub trait Transport {
    type Input;   // e.g., Command
    type Output;  // e.g., Response

    async fn read_message(&mut self) -> io::Result<Option<Self::Input>>;
    async fn send_message(&mut self, msg: Self::Output) -> io::Result<()>;
}


pub mod stdio;
