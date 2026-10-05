#![allow(nonstandard_style)]

// Imports
use std::{
    net::{IpAddr, UdpSocket}, sync::{LazyLock, Mutex},
};

use serde::{Deserialize, Serialize};

pub mod peers;
pub mod folderWatcher;
pub mod manifest;
pub mod sync;

// Variables
pub static SYNC_FOLDER_LOCATION: &str = "/home/ishank/bixsync";

pub const PORT: u16 = 2637;
pub const PEERS_FILE: &str = "peers.json";
pub const MANIFEST_FILE: &str = "manifest.json";

pub static mut Peers: Vec<String> = Vec::new();

pub static IgnoreFileSync: Mutex<Vec<String>> = Mutex::new(vec![]);

pub static SelfIpAddr: LazyLock<IpAddr> = LazyLock::new(|| {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap();
    socket.local_addr().unwrap().ip()
});

// Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestStructure {
    file: String,
    updateId: i32,
}