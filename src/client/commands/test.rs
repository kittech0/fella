use lib::Command;
use serenity::async_trait;

use super::TestCommand;

#[async_trait]
impl Command for TestCommand {
    async fn run(&self) ->  anyhow::Result<()> {
        todo!()
    }
}
