use std::env;

use lib::BotManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let manager = BotManager::builder()
        .set_token(env::var("BOT_TOKEN")?)
        .build()?;
    Ok(())
}
