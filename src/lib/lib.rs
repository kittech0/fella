#![feature(async_fn_traits)]
use std::{collections::HashMap, ops::AsyncFn, sync::Arc};

use async_trait::async_trait;

pub mod command;
pub mod error;
pub mod manager;
pub struct BotManager {
    token: String,
    commands: HashMap<AStr, CommandWrapper>,
}

pub struct BotBuilder {
    token: Option<String>,
    commands: HashMap<AStr, CommandWrapper>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct AStr(Arc<str>);

#[derive(Clone)]
pub struct CommandWrapper {
    description: AStr,
    command: Arc<dyn Command + Sync + Send>,
}

#[async_trait]
pub trait Command {
    async fn run(&self) -> anyhow::Result<()>;
}
