mod disconnect;
mod event;
mod key_exchange;
mod read;
mod test_connection;
mod write;

use std::collections::HashMap;
use std::net::ToSocketAddrs;

use log::{debug, info, trace, warn};
use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use snafu::{ResultExt, Snafu};

/// A token the allows us to identify which event is for which socket.
const REGISTRAR: Token = Token(0);

/// Errors one might get when running the participant.
#[derive(Debug, Snafu)]
pub enum RegistrarError {
    /// Failed to construct a new poll.
    #[snafu(display("Failed to create a new poll. {}", source))]
    NewPollFailure { source: std::io::Error },

    /// Failed to poll.
    #[snafu(display("Failed to poll address {}. {}", addr, source))]
    PollFailure {
        addr: String,
        source: std::io::Error,
    },

    /// Failed to parse a hostname string.
    #[snafu(display(
        "Failed to parse address {}. {}. Please use CIDR notation!",
        addr,
        source
    ))]
    AddressParseFailure {
        addr: String,
        source: std::io::Error,
    },

    /// Failed to bind to a ip and port combonation.
    #[snafu(display("Failed to bind to address {} on the network. {}", addr, source))]
    BindFailure {
        addr: String,
        source: std::io::Error,
    },

    /// Failed to register a connection.
    #[snafu(display("Failed to register a connection from {}. {}", conn, source))]
    ConnectionRegisterFailure {
        conn: String,
        source: std::io::Error,
    },
}

/// State required to run the registrar.
#[derive(Debug)]
pub struct Registrar {
    poll: Poll,
    server: TcpListener,
}

impl Registrar {
    /// Create a new registrar from an address string.
    /// The address string should be in CIDR notation.
    pub fn from_addr(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        trace!("Creating from addr {}", addr);

        let parsed = addr.to_socket_addrs().context(AddressParseFailure{addr: &addr})?
            .next()
            .expect("Somehow parsing the hostname succeeded, but to_socket_addrs didn't populate anything. THIS IS A BUG!");
        let poll = Poll::new().context(NewPollFailure {})?;
        let mut server = TcpListener::bind(parsed).context(BindFailure { addr })?;

        poll.registry()
            .register(&mut server, REGISTRAR, Interest::READABLE)?;

        Ok(Self { poll, server })
    }

    /// Run the registrar.
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut connections = HashMap::new();
        let mut unique_token = Token(REGISTRAR.0 + 1);
        let mut events = Events::with_capacity(128);

        info!("The CatNet registrar is all set up! (Note that this doesn't mean a stateless application)");
        info!("You can connect via a CatNet driver set to participant mode. You can also view network status");
        info!("via this very tool! (this is a seperate subcommand)");

        loop {
            self.poll.poll(&mut events, None)?;

            for event in events.into_iter() {
                trace!("Received NEW event {:?}", event);
                match event.token() {
                    REGISTRAR => {
                        loop {
                            // Received an event for a TCP server socket, which means
                            // that we, yes, finally, can accept a connection!
                            let (mut connection, address) = match self.server.accept() {
                                Ok((connection, address)) => (connection, address),
                                Err(e) => {
                                    if e.kind() == std::io::ErrorKind::WouldBlock {
                                        // We now know our listener has no more incoming connections queued,
                                        // so we can return to polling and wait for some more.
                                        break;
                                    }

                                    // ):
                                    // It was another kind of error, something went horribly, unbelievably
                                    // wrong. We can still continue, since this has nothing to do with the
                                    // registrar itself. We just don't respond, and the participant gets
                                    // the memo.
                                    warn!("An unknown error of type {:?} was encountered, not accepting.", e.kind());
                                    break;
                                }
                            };

                            info!("Accepted connection from a participant(?) at {}!", address);

                            // Register the connection.
                            let token = next(&mut unique_token);
                            self.poll
                                .registry()
                                .register(
                                    &mut connection,
                                    token,
                                    Interest::READABLE.add(Interest::WRITABLE),
                                )
                                .context(ConnectionRegisterFailure {
                                    conn: &address.to_string(),
                                })?;

                            connections.insert(token, connection);
                        }
                    }

                    mut token => {
                        // Quite possibly a TCP event from an ongoing connection.
                        // Its not for certain, but it looks promising.

                        let connection = match connections.get_mut(&mut token) {
                            Some(c) => c,
                            None => {
                                warn!("No token found for incomming connection. Connection should be registered, but the oposite is true.");
                                continue;
                            }
                        };

                        debug!("Handling event.");
                        self.handle_event(connection, event);
                    }
                }
            }
        }
    }
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;

    Token(next)
}
