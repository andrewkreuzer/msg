use anyhow::Result;

use msg::server::Server;

#[tokio::main]
pub async fn main() -> Result<()> {
    let server = Server::new().await;
    server.run().await?;
    Ok(())
}
