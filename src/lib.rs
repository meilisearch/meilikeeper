use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Token};

// Some tokens to allow us to identify which event is for which socket.
const CLIENT: Token = Token(0);

pub struct Zookeeper {
    hostname: String, // it will be a list in the future
    watchers: HashMap<WatchInfo, Sender<anyhow::Result<Event>>>,
    join_handle: JoinHandle<anyhow::Result<()>>,
}

impl Zookeeper {
    pub fn connect(hostname: impl Into<String>) -> anyhow::Result<Self> {
        let hostname = hostname.into();
        let addr = hostname.parse()?;
        let join_handle = thread::spawn(|| {
            let mut poll = Poll::new()?;
            // Create storage for events.
            let mut events = Events::with_capacity(128);

            // Setup the client socket.
            let mut client = TcpStream::connect(addr)?;
            // Register the socket.
            poll.registry().register(
                &mut client,
                CLIENT,
                Interest::READABLE | Interest::WRITABLE,
            )?;

            // Start an event loop.
            loop {
                // Poll Mio for events, blocking until we get an event.
                poll.poll(&mut events, None)?;

                // Process each event.
                for event in events.iter() {
                    // We can use the token we previously provided to `register` to
                    // determine for which socket the event is.
                    match event.token() {
                        CLIENT => {
                            if event.is_writable() {
                                // We can (likely) write to the socket without blocking.
                            }

                            if event.is_readable() {
                                // We can (likely) read from the socket without blocking.
                            }

                            // Since the server just shuts down the connection, let's
                            // just exit from our event loop.
                            return Ok(());
                        }
                        // We don't expect any events with tokens other than those we provided.
                        _ => unreachable!(),
                    }
                }
            }
        });

        Ok(Zookeeper {
            hostname,
            watchers: Default::default(),
            join_handle,
        })
    }

    pub fn watch(
        &self,
        path: impl Into<String>,
    ) -> anyhow::Result<Receiver<anyhow::Result<Event>>> {
        let (sender, receiver) = mpsc::channel();
        self.write_stream.write_all(&[1, 2, 3, 4])?;
        self.watchers
            .insert(WatchInfo { path: path.into() }, sender);
        Ok(receiver)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WatchInfo {
    path: String,
}

enum Event {
    NodeCreated,
    NodeDeleted,
}
