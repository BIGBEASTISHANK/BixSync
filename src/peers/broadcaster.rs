use std::{net::UdpSocket, thread, time::Duration};

pub fn init() {
    let SOCKET = UdpSocket::bind("0.0.0.0:0").unwrap();

    println!("UDP broadcaster started");

    SOCKET.set_broadcast(true).unwrap();

    loop {
        let MSG = format!(r#"{{"app":"bixsync","port":{}}}"#, crate::PORT);

        let _ = SOCKET.send_to(MSG.as_bytes(), format!("255.255.255.255:{}", crate::PORT));

        thread::sleep(Duration::from_secs(5));
    }
}
