use std::io;

use super::{super::message::{self, base}, Registrar};

use log::{debug, error, trace};
use mio::event::Event;
use mio::net::TcpStream;
use snafu::ResultExt;

impl Registrar {
    pub fn handle_event(&mut self, connection: &mut TcpStream, event: &Event) {
        debug!("Got new event (Event: {:?}).", connection);

        if event.is_readable() {
            let response = match self.read(connection, event).map_err(|e| {
                error!("Failed to read from connection. {}", e);
                return;
            }) {
                Ok(r) => r,
                Err(_) => return,
            };

            let message: base::Message = match bincode::deserialize(&response.0)
                .context(message::DeserializeFailure {})
            {
                Ok(m) => m,
                Err(e) => {
                    error!("Failed to deserialize message. {}", e);
                    match self.write_data(
                        "deserialization failure".as_bytes().to_vec(),
                        connection,
                        event,
                    ) {
                        Ok(()) => (),
                        Err(e) => error!("Failed to send deserialization failure message. {}", e),
                    };

                    return;
                }
            };

            trace!("Received valid message:\n{:?}", &message);

            match message.command {
                base::Command::TestConnection() => {
                    let msg = self.test_connection(message);
                    trace!("Sending:\n{:?}", msg);
                    match self.write_data(
                        match bincode::serialize(&msg) {
                            Ok(d) => d,
                            Err(e) => {
                                error!("Failed to serialize message. {}", e);
                                return;
                            }
                        },
                        connection,
                        event,
                    ) {
                        Ok(()) => (),
                        Err(e) => {
                            error!("Failed to send data to participant. {}", e);
                            return;
                        }
                    };

                    trace!("Sent!");
                }

                base::Command::PubkeyExchange() => {
                    self.exchange(message);
                }

                base::Command::DisconnectToken { token: _ } => {
                    self.disconnect(message);
                }
            }
        }
    }
}

pub fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

pub fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
