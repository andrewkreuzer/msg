pub mod server;
pub mod client;
pub mod message;
pub mod state;
pub mod user;

pub async fn run() -> anyhow::Result<()> {
    println!("Hello, world!");

    Ok(())
}

