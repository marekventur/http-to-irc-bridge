use std::net::SocketAddrV4;
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::thread::{spawn, sleep_ms};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::io::Read;

use IrcChannelTextMessage;

use hyper::server::{Server,Request, Response};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;
use hyper::method::Method;
use std::sync::Mutex;
use regex::Regex;

pub struct HttpServer {
    sock_addr: SocketAddrV4,
}

impl HttpServer {
    pub fn new(port: u16, host: String) -> HttpServer {
        let host_as_str: &str = &host[..];
        HttpServer{sock_addr: SocketAddrV4::new(FromStr::from_str(host_as_str).unwrap(), port)}
    }

   /* fn handle_request(&self, req: Request, mut res: Response, tx: Sender<IrcChannelTextMessage>) {

    }*/

    pub fn start(self, tx: Sender<IrcChannelTextMessage>, alive: Arc<AtomicBool>) -> JoinHandle<()> {
        spawn(move || {
            println!("Starting HTTP server on {}", self.sock_addr);

            let path_regex = Regex::new(r"^/([^/]+)$|^/private/([^/]+)$").unwrap();

            let tx_mutex = Mutex::new(tx);
            let server = Server::http(move |req: Request, mut res: Response| {
                let (status_code, body) = match req.deconstruct() {
                (_, Method::Post, _, RequestUri::AbsolutePath(path), _, mut body) => {
                    match path_regex.captures(&path[..]) {
                        Some(captures) => {
                            let channel = match (captures.at(1), captures.at(2)) {
                                (Some(channel), _) => format!("#{}", channel),
                                (_, Some(username)) => username.to_owned(),
                                _ => unreachable!(),
                            };

                            let mut text = String::new();
                            body.read_to_string(&mut text).unwrap();
                            let message = IrcChannelTextMessage{channel: channel, text: text};
                            match tx_mutex.lock().unwrap().send(message) {
                                Ok(_) => (StatusCode::Ok, "Ok"),
                                Err(err) => {
                                    println!("Error: {}", err);
                                    (StatusCode::InternalServerError, "Internal Server Error")
                                }
                            }
                        },
                        None => (StatusCode::BadRequest, "Only requests to /private/{user} and /{channel}"),
                    }

                },
                _ => (StatusCode::MethodNotAllowed, "Only POST allowed"),
        };

        *res.status_mut() = status_code;
        res.send(body.as_bytes()).unwrap();
            });

            match server.listen(self.sock_addr) {
                Ok(mut listening) => {
                    println!("HTTP Server started");
                    // Busy wait. Ugh.
                    while alive.load(SeqCst) {
                        sleep_ms(100);
                    }

                    println!("Stopping HTTP server...");
                    listening.close().unwrap();
                    println!("HTTP server stopped");
                },
                Err(err) => {
                    println!("Error: {}", err);
                    alive.store(false, SeqCst);
                }
            }
        })
    }
}

