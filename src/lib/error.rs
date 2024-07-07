use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to build BotManager")]
    BotBuilderError(#[from] BotBuilderError),
}

#[derive(Error, Debug)]
pub enum BotBuilderError {
    #[error("Undefined token")]
    UndefinedTokenError
}
