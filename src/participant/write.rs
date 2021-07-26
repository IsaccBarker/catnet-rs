use std::io::Write;

use super::{Participant, super::message::{self, Message}};

use snafu::{Snafu, ResultExt};
use log::trace;

#[derive(Debug, Snafu)]
pub enum WriteError {
    #[snafu(display("Failed to write data to registrar TCP stream. {}", source))]
    RegistrarWriteError {
        source: std::io::Error,
    }
}

impl Participant {
    pub fn send_message(&mut self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        let data = bincode::serialize(&message).context(message::SerializeFailure{})?;
        self.send_data(data)?;
        
        Ok(())
    }

    pub fn send_data(&mut self, data: Vec<u8>) -> Result<(), WriteError> {
        trace!("Sending:\n{:?}", data); 
        self.stream.write(data.as_slice()).context(RegistrarWriteError{})?;

        Ok(())
    }
}

