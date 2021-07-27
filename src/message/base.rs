use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    TestConnection(),
    PubkeyExchange(),
    DisconnectToken { token: usize },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub command: Command,
    pub response: Vec<String>,
}

impl Message {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            response: vec![],
        }
    }
}

