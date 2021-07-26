use mio::event::Event;
use mio::net::TcpStream;
use mio::Registry;
use log::{trace, debug};

use std::io::Read;
use std::str::from_utf8;

use super::{Registrar, event};

impl Registrar {
    /// Reads data from the connection.
    /// Returns true if the connection is closed.
    pub fn read(
        &mut self,
        connection: &mut TcpStream,
        _event: &Event,
    ) -> std::io::Result<(Vec<u8>, bool)> {
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;

        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    break;
                }
                
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
            
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                Err(ref err) if event::would_block(err) => break,
                Err(ref err) if event::interrupted(err) => continue,
                // Other errors we'll consider fatal.
                Err(err) => return Err(err)
            } 
        }

        if bytes_read != 0 {
            let received_data = &received_data[..bytes_read];
        
            if let Ok(str_buf) = from_utf8(received_data) {
                trace!("Received data: {}", str_buf.trim_end());
            } else {
                trace!("Received (none UTF-8) data: {:?}", received_data);
            }
        }

        Ok((received_data, false))
    }
}
