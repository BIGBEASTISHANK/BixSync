use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn receive_file(tcp: &mut TcpStream, file: &mut File, size: u64) -> io::Result<()> {
    let mut remaining = size;
    let mut buf = [0u8; 4096];

    while remaining > 0 {
        let to_read = std::cmp::min(remaining, buf.len() as u64) as usize;

        let n = tcp.read(&mut buf[..to_read])?;

        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "connection closed while receiving file",
            ));
        }

        file.write_all(&buf[..n])?;

        remaining -= n as u64;
    }

    Ok(())
}

fn handle_client(mut tcp: TcpStream) -> io::Result<()> {
    // Receive sizes
    let mut size_buf = [0u8; 8];

    tcp.read_exact(&mut size_buf)?;
    let sync_size = u64::from_be_bytes(size_buf);

    tcp.read_exact(&mut size_buf)?;
    let manifest_size = u64::from_be_bytes(size_buf);

    println!("sync file: {} bytes", sync_size);
    println!("manifest file: {} bytes", manifest_size);

    // Receive sync file
    let mut sync_file = File::create("received_sync_file")?;

    receive_file(&mut tcp, &mut sync_file, sync_size)?;

    println!("Sync file received");

    // Receive manifest file
    let mut manifest_file = File::create("received_manifest_file")?;

    receive_file(&mut tcp, &mut manifest_file, manifest_size)?;

    println!("Manifest file received");

    Ok(())
}

pub fn init() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4321")?;

    println!("Waiting for connection...");

    for stream in listener.incoming() {
        let stream = stream?;

        println!("Client connected");

        handle_client(stream)?;
    }

    Ok(())
}
