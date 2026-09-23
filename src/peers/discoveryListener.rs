use std::net::UdpSocket;

use crate::peers;

pub fn init() {
    let mut peers = peers::LoadPears();

    let SOCKET = UdpSocket::bind(format!("0.0.0.0:{}", crate::PORT)).unwrap();

    let mut buf = [0u8; 1024];

    println!("UDP listener started");

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

        println!("Discovered {} -> {}", IP, MSG);

        peers.push(IP);

        peers::SavePeers(&peers);
    }
}
