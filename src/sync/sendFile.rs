use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use colored::Color;

pub fn init(PATH: String, ipAddr: String) -> io::Result<()> {
    let mut syncfile = File::open(&PATH)?;
    let mut manifestFile = File::open(&crate::MANIFEST_FILE)?;

    let ADDR = format!("{}:{}", ipAddr, crate::PORT)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid address"))?;

    // TCP Connection with timeout
    let mut tcp = TcpStream::connect_timeout(&ADDR, Duration::from_secs(5))?;

    // Get file sizes
    let SYNC_SIZE = syncfile.metadata()?.len();
    let MANIFEST_SIZE = manifestFile.metadata()?.len();

    let FILE_NAME = &PATH.replace(crate::SYNC_FOLDER_LOCATION, "")[1..];

    // Debug logs
    println!();
    crate::DebugLog(
        &format!("Started sending file to {}", ipAddr),
        Color::Yellow,
    );
    println!("###############");

    // Send sizes first
    tcp.write_all(&SYNC_SIZE.to_be_bytes())?;
    crate::DebugLog(
        format!("Sent file size: {} bytes", SYNC_SIZE).as_str(),
        Color::Yellow,
    );
    tcp.write_all(&MANIFEST_SIZE.to_be_bytes())?;
    crate::DebugLog(
        format!("Sent manifest size: {} bytes", MANIFEST_SIZE).as_str(),
        Color::Yellow,
    );
    tcp.write_all(&FILE_NAME.len().to_be_bytes())?;
    crate::DebugLog(
        format!("Sent file name size: {} bytes", FILE_NAME.len()).as_str(),
        Color::Yellow,
    );
    tcp.write_all(&crate::MANIFEST_FILE.len().to_be_bytes())?;
    crate::DebugLog(
        format!(
            "Sent manifest name size: {} bytes",
            crate::MANIFEST_FILE.len()
        )
        .as_str(),
        Color::Yellow,
    );

    // Sending File names
    tcp.write_all(FILE_NAME.as_bytes())?;
    crate::DebugLog(
        format!("Sent file name: {}", FILE_NAME).as_str(),
        Color::Yellow,
    );
    tcp.write_all(crate::MANIFEST_FILE.as_bytes())?;
    crate::DebugLog(
        format!("Sent manifest name: {}", crate::MANIFEST_FILE).as_str(),
        Color::Yellow,
    );

    let mut buf = [0u8; 4096];

    // Send manifest file
    loop {
        let ITER = manifestFile.read(&mut buf)?;

        if ITER == 0 {
            break;
        }

        tcp.write_all(&buf[..ITER])?;
    }

    // Send sync file
    loop {
        let ITER = syncfile.read(&mut buf)?;

        if ITER == 0 {
            break;
        }

        tcp.write_all(&buf[..ITER])?;
    }

    Ok(())
}
