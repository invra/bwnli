use config::load_config;
mod https;
mod ws;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let config = load_config();
    // _ = https::start().await;
    _ = ws::start(config.servers.websockets.into()).await;
    Ok(())
}
