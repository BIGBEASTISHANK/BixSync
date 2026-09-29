use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;

pub fn init(path: String) -> io::Result<()> {
    let mut syncfile = File::open(path)?;
    let mut manifestFile = File::open(crate::MANIFEST_FILE)?;

    let mut tcp = TcpStream::connect("192.168.1.3:4321")?;

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
