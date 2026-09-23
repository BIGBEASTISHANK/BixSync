#![allow(nonstandard_style)]

// Imports
use std::{
    net::{IpAddr, UdpSocket},
    sync::LazyLock,
};

pub mod peers;
pub mod folderWatcher;

// Variables
pub static SYNC_FOLDER_LOCATION: &str = "/home/ishank/bixsync";

pub const PORT: u16 = 2637;
pub const PEERS_FILE: &str = "peers.json";

pub static mut Peers: Vec<String> = Vec::new();

pub static SelfIpAddr: LazyLock<IpAddr> = LazyLock::new(|| {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap();
    socket.local_addr().unwrap().ip()
});