pub mod base;
pub mod catnip;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum SerdeError {
    #[snafu(display("Failed to deserialize message into Message struct. {}", source))]
    DeserializeFailure { source: bincode::Error },

    #[snafu(display("Failed to serialize message from Message struct. {}", source))]
    SerializeFailure { source: bincode::Error },
}

