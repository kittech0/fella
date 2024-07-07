use std::{ops::AsyncFn, sync::Arc};

use async_trait::async_trait;

use crate::{AStr, Command, CommandWrapper};

impl From<&str> for AStr {
    fn from(value: &str) -> Self {
        Self(Arc::from(value))
    }
}

impl AsRef<str> for AStr {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl CommandWrapper {
    pub fn new<D: AsRef<str>, C: Command + Sync + Send + 'static>(description: D, command: C) -> Self {
        Self {
            description: description.as_ref().into(),
            command: Arc::from(command),
        }
    }
}
