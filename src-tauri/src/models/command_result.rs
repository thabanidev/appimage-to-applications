use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
    pub log: Vec<String>,
}

impl CommandResult {
    pub fn ok(message: impl Into<String>, log: Vec<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            log,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            log: Vec::new(),
        }
    }
}
