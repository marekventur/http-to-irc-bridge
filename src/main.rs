extern crate hyper;
extern crate irc;
extern crate regex;

mod http_server;
mod irc_bot;

use irc::client::data::Config;
use std::str::FromStr;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

struct IrcChannelTextMessage {
    channel: String,
    text: String
}

fn main() {
    let alive = Arc::new(AtomicBool::new(true));

    let irc_config: Config = Config::load_utf8("config.json").unwrap();
    let (http_port, http_host) = get_port_and_host(&irc_config);

    let irc = irc_bot::IrcBot::new(irc_config);
    let http = http_server::HttpServer::new(http_port, http_host);

    let (tx, rx): (Sender<IrcChannelTextMessage>, Receiver<IrcChannelTextMessage>) = mpsc::channel();

    let irc_handler = irc.start(rx, alive.clone());
    let http_handler = http.start(tx, alive.clone());

    http_handler.join().unwrap();
    irc_handler.join().unwrap();
}

fn get_port_and_host(config: &Config) -> (u16, String) {
    let port: u16 = FromStr::from_str(&config.get_option("http_port")).unwrap();
    let mut host: String = String::new();
    host.push_str(config.get_option("http_host"));
    (port, host)
}