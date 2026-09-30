use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn init(path: String, ipAddr: String) -> io::Result<()> {
    let mut syncfile = File::open(path)?;
    let mut manifestFile = File::open(crate::MANIFEST_FILE)?;

    let ADDR = format!("{}:{}", ipAddr, crate::PORT)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid address"))?;

    // TCP Connection with timeout
    let mut tcp = TcpStream::connect_timeout(&ADDR, Duration::from_secs(5))?;

    // Get file sizes
    let SYNC_SIZE = syncfile.metadata()?.len();
    let MANIFEST_SIZE = manifestFile.metadata()?.len();

    // Send sizes first
    tcp.write_all(&SYNC_SIZE.to_be_bytes())?;
    tcp.write_all(&MANIFEST_SIZE.to_be_bytes())?;

    let mut buf = [0u8; 4096];

    // Send sync file
    loop {
        let ITER = syncfile.read(&mut buf)?;

        if ITER == 0 {
            break;
        }

        tcp.write_all(&buf[..ITER])?;
    }

    // Send manifest file
    loop {
        let ITER = manifestFile.read(&mut buf)?;

        if ITER == 0 {
            break;
        }

        tcp.write_all(&buf[..ITER])?;
    }

    Ok(())
}
