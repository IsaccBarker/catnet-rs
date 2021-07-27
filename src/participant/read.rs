use std::io::Read;

use super::{
    super::message::{self, base},
    Participant,
};

use log::trace;
use snafu::{ResultExt, Snafu};

/// Errors that can be raised when the participant is
/// attempting to read from the participant <-> registrar
/// socket.
#[derive(Debug, Snafu)]
pub enum ReadError {
    /// Failed to read data from the registrar TCP stream.
    #[snafu(display("Failed to read data from registrar TCP stream. {}", source))]
    RegistrarReadError { source: std::io::Error },
}

impl Participant {
    /// Receives a message. First calls receive_data, then deserializes it into
    /// a usefull message.
    pub fn receive_message(&mut self) -> Result<base::Message, Box<dyn std::error::Error>> {
        let data = self.receive_data()?;
        let message = bincode::deserialize(&data).context(message::SerializeFailure {})?;

        Ok(message)
    }

    /// Receive raw data from the socket.
    pub fn receive_data(&mut self) -> Result<Vec<u8>, ReadError> {
        let buffer_size = 1024;
        let mut recv: Vec<u8> = vec![];
        // self.stream.read(recv.as_mut_slice()).context(RegistrarReadError{})?;

        loop {
            let mut buf = vec![0 as u8; buffer_size];
            let read = self.stream.read(&mut buf).context(RegistrarReadError {})?;

            recv.append(&mut buf);
            if read != buffer_size {
                break;
            }
        }

        trace!("Received:\n{:?}", recv);

        Ok(recv)
    }
}
