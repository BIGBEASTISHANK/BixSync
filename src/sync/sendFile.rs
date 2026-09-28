use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;

pub fn init(path: String) -> io::Result<()> {
    let mut syncfile = File::open(path)?;
    let mut manifest_file = File::open(crate::MANIFEST_FILE)?;

    let mut tcp = TcpStream::connect("192.168.1.3:4321")?;

    // Get file sizes
    let sync_size = syncfile.metadata()?.len();
    let manifest_size = manifest_file.metadata()?.len();

    // Send sizes first
    tcp.write_all(&sync_size.to_be_bytes())?;
    tcp.write_all(&manifest_size.to_be_bytes())?;

    let mut buf = [0u8; 4096];
    
    // Send sync file
    loop {
        let n = syncfile.read(&mut buf)?;

        if n == 0 {
            break;
        }

        tcp.write_all(&buf[..n])?;
    }

    // Send manifest file
    loop {
        let n = manifest_file.read(&mut buf)?;

        if n == 0 {
            break;
        }

        tcp.write_all(&buf[..n])?;
    }

    Ok(())
}
