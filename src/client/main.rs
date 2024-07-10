use std::env;

use commands::TestCommand;
use lib::BotManager;
mod commands;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let manager = BotManager::builder()
        .set_token(env::var("BOT_TOKEN")?)
        .add_command("test", "Testing command", TestCommand)
        .build()?;
    Ok(())
}
