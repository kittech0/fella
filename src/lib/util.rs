use std::sync::Arc;


use crate::AStr;


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

impl ToString for AStr {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
