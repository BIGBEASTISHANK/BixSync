use std::net::UdpSocket;

use colored::Color;

use crate::peers;

pub fn init() {
    let mut peers = peers::LoadPears();

    let SOCKET = UdpSocket::bind(format!("0.0.0.0:{}", crate::PORT)).unwrap();

    let mut buf = [0u8; 1024];

    // Debug Logs
    println!("UDP listener started");
    crate::ThreadReady();

    loop {
        let (SIZE, SENDER) = SOCKET.recv_from(&mut buf).unwrap();

        let MSG = String::from_utf8_lossy(&buf[..SIZE]);

        if !MSG.contains("\"app\":\"bixsync\"") {
            continue;
        }

        let IP = SENDER.ip().to_string();

        if IP == crate::SelfIpAddr.to_string() || peers.contains(&IP) {
            continue;
        }

        crate::DebugLog(&format!("Discovered {} -> {}", IP, MSG), Color::Green);

        peers.push(IP);

        peers::SavePeers(&peers);
    }
}
