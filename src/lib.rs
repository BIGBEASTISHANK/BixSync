#![allow(nonstandard_style)]

// Imports
use colored::{Color, Colorize};
use std::{
    net::{IpAddr, UdpSocket},
    sync::{LazyLock, Mutex},
};

use serde::{Deserialize, Serialize};

pub mod folderWatcher;
pub mod manifest;
pub mod peers;
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

// Debug Variables
#[cfg(debug_assertions)]
static ThreadCounter: Mutex<i8> = Mutex::new(0);

// Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestStructure {
    file: String,
    updateId: i32,
}

// Functions
#[cfg(debug_assertions)]
pub fn DebugLog(msg: &str, color: Color) {
    println!("{} {}","[DEBUG]".color(Color::Blue), msg.color(color));
}

#[cfg(debug_assertions)]
pub fn ThreadReady() {
    let mut count = ThreadCounter.lock().unwrap();
    *count += 1;

    if *count == 4 {
        // Debug Logsc
        DebugLog("Thread starging finished", Color::Yellow);
    }
}
