use std::io::Read;

use super::{Participant, super::message::{self, Message}};

use log::trace;
use snafu::{Snafu, ResultExt};

#[derive(Debug, Snafu)]
pub enum ReadError {
    #[snafu(display("Failed to read data from registrar TCP stream. {}", source))]
    RegistrarReadError {
        source: std::io::Error,
    }
}

impl Participant {
    pub fn receive_message(&mut self) -> Result<Message, Box<dyn std::error::Error>> {
        let data = self.receive_data()?;
        let message = bincode::deserialize(&data).context(message::SerializeFailure{})?;

        Ok(message)
    }

    pub fn receive_data(&mut self) -> Result<Vec<u8>, ReadError> {
        let buffer_size = 1024;
        let mut recv: Vec<u8> = vec![];
        // self.stream.read(recv.as_mut_slice()).context(RegistrarReadError{})?;

        loop {
            let mut buf = vec![0 as u8; buffer_size];
            let read = self.stream.read(&mut buf).context(RegistrarReadError{})?;

            recv.append(&mut buf);
            if read != buffer_size {
                break;
            }
        }

        trace!("Received:\n{:?}", recv);

        Ok(recv)
    }
}

