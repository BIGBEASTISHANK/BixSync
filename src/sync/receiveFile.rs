use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn receiveFile(tcp: &mut TcpStream, file: &mut File, SIZE: u64) -> io::Result<()> {
    let mut remaining = SIZE;
    let mut buf = [0u8; 4096];

    while remaining > 0 {
        let TO_READ = std::cmp::min(remaining, buf.len() as u64) as usize;

        let ITER = tcp.read(&mut buf[..TO_READ])?;

        if ITER == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "connection closed while receiving file",
            ));
        }

        file.write_all(&buf[..ITER])?;

        remaining -= ITER as u64;
    }

    Ok(())
}

fn handleClient(mut tcp: TcpStream) -> io::Result<()> {
    // Receive sizes
    let mut sizeBuff = [0u8; 8];

    tcp.read_exact(&mut sizeBuff)?;
    let SYNC_SIZE = u64::from_be_bytes(sizeBuff);

    tcp.read_exact(&mut sizeBuff)?;
    let MANIFEST_SIZE = u64::from_be_bytes(sizeBuff);

    tcp.read_exact(&mut sizeBuff)?;
    let PATH_SIZE = u64::from_be_bytes(sizeBuff);

    tcp.read_exact(&mut sizeBuff)?;
    let MANIFEST_PATH_SIZE = u64::from_be_bytes(sizeBuff);

    // Receive file names
    let mut pathNameBuff = vec![0u8; PATH_SIZE as usize];
    let mut manifestNameBuff = vec![0u8; MANIFEST_PATH_SIZE as usize];

    tcp.read_exact(&mut pathNameBuff)?;
    tcp.read_exact(&mut manifestNameBuff)?;

    let PATH_NAME = String::from_utf8(pathNameBuff)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let MANIFEST_NAME = String::from_utf8(manifestNameBuff)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    println!("Path Name: {:?}", PATH_NAME);
    println!("Manifest Name: {:?}", MANIFEST_NAME);

    println!("sync file: {} bytes", SYNC_SIZE);
    println!("manifest file: {} bytes", MANIFEST_SIZE);

    // Receive sync file
    let mut syncFile = File::create(format!("{}.bixsync", PATH_NAME))?;

    receiveFile(&mut tcp, &mut syncFile, SYNC_SIZE)?;

    println!("Sync file received");

    // Receive manifest file
    let mut manifestFile = File::create(format!("{}.bixsync", MANIFEST_NAME))?;

    receiveFile(&mut tcp, &mut manifestFile, MANIFEST_SIZE)?;

    println!("Manifest file received");

    Ok(())
}

pub fn init() -> io::Result<()> {
    let listener = TcpListener::bind(format! {"0.0.0.0:{}", crate::PORT})?;

    println!("Waiting for connection...");

    for STREAM in listener.incoming() {
        let STREAM = STREAM?;

        println!("Client connected");

        handleClient(STREAM)?;
    }

    Ok(())
}
