use std::collections::HashMap;

use crate::{error::BotBuilderError, AStr, BotBuilder, BotManager, Command, CommandWrapper};

impl BotManager {
    pub fn builder() -> BotBuilder {
        BotBuilder::new()
    }
}

impl BotBuilder {
    fn new() -> Self {
        Self {
            token: Option::None,
            commands: HashMap::new(),
        }
    }

    pub fn build(self) -> Result<BotManager, BotBuilderError> {
        Ok(BotManager {
            token: self.token.ok_or(BotBuilderError::UndefinedTokenError)?,
            commands: self.commands,
        })
    }

    pub fn set_token<S: Into<String>>(mut self, token: S) -> Self {
        self.token = Option::Some(token.into());
        self
    }

    pub fn add_command<N: AsRef<str>, D: AsRef<str>, C: Command + Sync + Send + 'static>(
        mut self,
        name: N,
        description: D,
        command: C,
    ) -> Self {
        self.commands.insert(
            name.as_ref().into(),
            CommandWrapper::new(description, command),
        );
        self
    }

    pub fn set_commands(mut self, commands: HashMap<AStr, CommandWrapper>) -> Self {
        self.commands = commands;
        self
    }
}
