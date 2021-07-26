use serde::{Serialize, Deserialize};
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum MessageError {
    #[snafu(display("Failed to deserialize message into Message struct. {}", source))]
    DeserializeFailure {
        source: bincode::Error
    },

    #[snafu(display("Failed to serialize message from Message struct. {}", source))]
    SerializeFailure {
        source: bincode::Error,
    },
}

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

