use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

use crate::manifest::compareManifest;

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

    // Setting it to ignore file sync
    crate::IgnoreFileSync
        .lock()
        .unwrap()
        .push(PATH_NAME.to_string());

    println!("sync file: {} bytes", SYNC_SIZE);
    println!("manifest file: {} bytes", MANIFEST_SIZE);

    // Receive manifest file
    let mut manifestFile = File::create(format!(
        "{}/{}.bixsync",
        crate::SYNC_FOLDER_LOCATION,
        MANIFEST_NAME
    ))?;

    receiveFile(&mut tcp, &mut manifestFile, MANIFEST_SIZE)?;
    drop(manifestFile);
    let mut manifestFile = File::open(format!(
        "{}/{}.bixsync",
        crate::SYNC_FOLDER_LOCATION,
        MANIFEST_NAME
    ))?;

    println!("Manifest file received");

    // Comparing manifest
    match compareManifest(&mut manifestFile, PATH_NAME.to_string()) {
        // Receive sync file
        Ok(_) => {
            let mut syncFile = File::create(format!(
                "{}/{}.bixsync",
                crate::SYNC_FOLDER_LOCATION,
                PATH_NAME
            ))?;

            // Receiving sync file
            receiveFile(&mut tcp, &mut syncFile, SYNC_SIZE)?;

            // Deleting manifest file & renaming file to original
            fs::remove_file(format!(
                "{}/{}.bixsync",
                crate::SYNC_FOLDER_LOCATION,
                MANIFEST_NAME
            ))?;
            fs::rename(
                format!("{}/{}.bixsync", crate::SYNC_FOLDER_LOCATION, PATH_NAME),
                format!("{}/{}", crate::SYNC_FOLDER_LOCATION, PATH_NAME),
            )?;

            println!("Sync file received");
        }
        Err(E) => {
            tcp.shutdown(Shutdown::Both)?;

            // Deleting files
            fs::remove_file(format!(
                "{}/{}.bixsync",
                crate::SYNC_FOLDER_LOCATION,
                PATH_NAME
            ))?;
            fs::remove_file(format!(
                "{}/{}.bixsync",
                crate::SYNC_FOLDER_LOCATION,
                MANIFEST_NAME
            ))?;

            println!("Failed to compare manifest: {E}");
            return Err(E);
        }
    }

    Ok(())
}

pub fn init() -> io::Result<()> {
    let listener = TcpListener::bind(format! {"0.0.0.0:{}", crate::PORT})?;

    // Debug Logs
    println!("Waiting for connection...");
    crate::ThreadReady();

    for STREAM in listener.incoming() {
        let STREAM = STREAM?;

        println!("Client connected");

        let _ = handleClient(STREAM).map_err(|E| {
            println!("Error while handeling client: {}", E);
        });
    }

    Ok(())
}
