use std::sync::Arc;

use crate::{Command, CommandWrapper};

impl CommandWrapper {
    pub fn new<N: AsRef<str>, D: AsRef<str>, C: Command + Sync + Send + 'static>(
        name: N,
        description: D,
        command: C,
    ) -> Self {
        Self {
            name: name.as_ref().into(),
            description: description.as_ref().into(),
            command: Arc::from(command),
        }
    }
}
