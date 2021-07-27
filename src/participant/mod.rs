mod read;
mod write;

use super::message::base;

use std::io;
use std::net::TcpStream;

use log::{debug, info};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum ParticipantError {
    #[snafu(display("Failed to connect to registrar at {}. {}", addr, source))]
    ConnectionFailure { addr: String, source: io::Error },
}

pub struct Participant {
    pub addr: String,
    pub stream: TcpStream,
}

impl Participant {
    pub fn from_addr(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(&addr).context(ConnectionFailure { addr: &addr })?;
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .expect("Failed to set read timeout on registrar TCP socket.");
        stream
            .set_write_timeout(Some(std::time::Duration::from_secs(5)))
            .expect("Failed to set right timeout on registrar TCP socket.");

        Ok(Self {
            addr: addr.clone(),
            stream,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Successfully connected to registrar at {}!", &self.addr);

        debug!("Sending test message.");
        self.send_message(base::Message::new(base::Command::TestConnection()))?;

        debug!("Sent! Receiving....");
        debug!("{:?}", self.receive_message()?);

        Ok(())
    }
}
