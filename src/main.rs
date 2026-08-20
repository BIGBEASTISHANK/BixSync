#![allow(nonstandard_style)]

use bixsync::*;
use std::{
    fs,
    io::{self},
    net::{UdpSocket},
    thread,
    time::Duration,
};

// Load peers
fn LoadPears() -> Vec<String> {
    if let Ok(TEXT) = fs::read_to_string(PEERS_FILE) {
        serde_json::from_str(&TEXT).unwrap_or_default()
    } else {
        vec![]
    }
}

// Save peers
fn SavePeers(PEERS: &[String]) {
    if let Ok(JSON) = serde_json::to_string_pretty(PEERS) {
        let _ = fs::write(PEERS_FILE, JSON);
    }
}

// Broadcaster
fn Broadcaster() {
    let SOCKET = UdpSocket::bind("0.0.0.0:0").unwrap();

    println!("UDP broadcaster started");

    SOCKET.set_broadcast(true).unwrap();

    loop {
        let MSG = format!(r#"{{"app":"bixsync","port":{}}}"#, PORT);

        let _ = SOCKET.send_to(MSG.as_bytes(), format!("255.255.255.255:{}", PORT));

        thread::sleep(Duration::from_secs(5));
    }
}

// Discovery listner
fn DiscoveryListner() {
    let mut peers = LoadPears();

    let SOCKET = UdpSocket::bind(format!("0.0.0.0:{}", PORT)).unwrap();

    let mut buf = [0u8; 1024];

    println!("UDP listener started");

    loop {
        let (SIZE, SENDER) = SOCKET.recv_from(&mut buf).unwrap();

        let MSG = String::from_utf8_lossy(&buf[..SIZE]);

        if !MSG.contains("\"app\":\"bixsync\"") {
            continue;
        }

        let IP = SENDER.ip().to_string();

        if IP == SelfIpAddr.to_string() || peers.contains(&IP) {
            continue;
        }

        println!("Discovered {} -> {}", IP, MSG);

        peers.push(IP);

        SavePeers(&peers);
    }
}

// -------------
// Main function
fn main() -> io::Result<()> {

    thread::spawn(Broadcaster);

    thread::spawn(DiscoveryListner);

    loop {
        thread::park();
    }
}