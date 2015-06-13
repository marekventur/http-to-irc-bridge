use std::sync::Arc;
use std::thread::spawn;
use irc::client::prelude::*;
use irc::client::server::NetIrcServer;
use std::thread::JoinHandle;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::sync::atomic::Ordering::SeqCst;
use std::result::Result::{Ok, Err};
use IrcChannelTextMessage;
use std::sync::{Mutex, Condvar};

pub struct IrcBot {
    server: NetIrcServer,
}

impl IrcBot {
    pub fn new(config: Config) -> IrcBot {
        let server = IrcServer::from_config(config).unwrap();

        IrcBot{
            server: server,
        }
    }

    fn start_connect_thread(server: Arc<NetIrcServer>, alive: Arc<AtomicBool>) -> JoinHandle<()> {
        // Thread 1: Connecting, Reconnecting, Processing input
        spawn(move || {
            while alive.load(SeqCst) {
                println!("Identifying to IRC");
                server.identify().unwrap();

                for msg in server.iter() {
                    match msg {
                        Ok(msg) => {
                            match Command::from_message(&msg) {
                                Ok(Command::ERROR(ref msg)) if msg.contains("Quit") => {
                                    alive.store(false, SeqCst);
                                }
                                _ => (),
                            }
                        },
                        Err(_)  => break,
                    }

                    if !alive.load(SeqCst) {
                        break
                    }
                }
                if alive.load(SeqCst) {
                    println!("Reconnecting IRC");
                    server.reconnect().unwrap();
                }
            }
        })
    }

    fn start_send_thread(server: Arc<NetIrcServer>, alive: Arc<AtomicBool>, rx: Receiver<IrcChannelTextMessage>) -> JoinHandle<()> {
        // Thread 2: Waiting for incoming messages, sending
        spawn(move || {
            while alive.load(SeqCst) {
                match rx.recv() {
                    Ok(IrcChannelTextMessage{ channel, text }) => {
                        server.send_privmsg(&channel[..], &text[..]).unwrap();
                    },
                    Err(err) => {
                        println!("Error caught when trying to receive message: {}", err);
                        alive.store(false, SeqCst);
                    }
                }
            };

            println!("Shutting down IRC connection");
            server.send_quit("").unwrap();
        })
    }

    pub fn start(self, rx: Receiver<IrcChannelTextMessage>, alive: Arc<AtomicBool>) -> JoinHandle<()> {
        let server = Arc::new(self.server);
        let handle1 = IrcBot::start_connect_thread(server.clone(), alive.clone());
        let handle2 = IrcBot::start_send_thread(server, alive.clone(), rx);

        spawn(move || {
            handle1.join().unwrap();
            handle2.join().unwrap();
        })
    }
}