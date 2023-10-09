use anyhow::Result;
use msg;

#[tokio::main]
async fn main() -> Result<()> {
    msg::run().await?;

    Ok(())
}
