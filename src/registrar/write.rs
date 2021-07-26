use mio::event::Event;
use mio::net::TcpStream;
use mio::Registry;
use log::{trace, debug};

use std::io::Write;
use std::str::from_utf8;

use super::{Registrar, event};

impl Registrar {
    /// Reads data from the connection.
    /// Returns true if the connection is closed.
    pub fn write_data(
        &mut self,
        data: Vec<u8>,
        connection: &mut TcpStream,
        event: &Event,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if event.is_writable() {
            connection.write_all(data.as_slice())?;
        }

        Ok(())
    }
}
