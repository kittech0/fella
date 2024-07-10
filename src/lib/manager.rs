use crate::{error::BotBuilderError, BotBuilder, BotManager, Command, CommandWrapper};

impl BotManager {
    pub fn builder() -> BotBuilder {
        BotBuilder::new()
    }
}

impl BotBuilder {
    fn new() -> Self {
        Self {
            token: Option::None,
            commands: Default::default(),
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
        let command_wrapper = CommandWrapper::new(name, description, command);
        self.commands.insert(
            command_wrapper.name.clone(),
            command_wrapper,
        );
        self
    }
}
