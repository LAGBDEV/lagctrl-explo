use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Plugin {
    async fn initialize(&self) -> Result<()>;
    async fn run(&self) -> Result<()>;
    async fn terminate(&self) -> Result<()>;
}
