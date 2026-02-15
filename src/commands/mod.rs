pub mod install;
pub mod post_upgrade;
pub mod repair;
pub mod status;
pub mod verify;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CommandReport {
    pub command: String,
    pub ok: bool,
    pub details: Vec<String>,
    pub issues: Vec<String>,
}

impl CommandReport {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            ok: true,
            details: Vec::new(),
            issues: Vec::new(),
        }
    }

    pub fn detail(&mut self, text: impl Into<String>) {
        self.details.push(text.into());
    }

    pub fn issue(&mut self, text: impl Into<String>) {
        self.ok = false;
        self.issues.push(text.into());
    }
}
