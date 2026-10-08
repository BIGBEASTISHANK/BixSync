use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

use colored::Color;

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

    // Debug logs
    println!();
    crate::DebugLog(&format!("Started receiving file from client: {}", tcp.peer_addr().unwrap()), Color::Yellow);

    tcp.read_exact(&mut sizeBuff)?;
    let SYNC_SIZE = u64::from_be_bytes(sizeBuff);
    crate::DebugLog(
        format!("Received file size: {} bytes", SYNC_SIZE).as_str(),
        Color::Green,
    );

    tcp.read_exact(&mut sizeBuff)?;
    let MANIFEST_SIZE = u64::from_be_bytes(sizeBuff);
    crate::DebugLog(
        format!("Received manifest size: {} bytes", MANIFEST_SIZE).as_str(),
        Color::Green,
    );

    tcp.read_exact(&mut sizeBuff)?;
    let PATH_SIZE = u64::from_be_bytes(sizeBuff);
    crate::DebugLog(
        format!("Received file name size: {} bytes", PATH_SIZE).as_str(),
        Color::Green,
    );

    tcp.read_exact(&mut sizeBuff)?;
    let MANIFEST_PATH_SIZE = u64::from_be_bytes(sizeBuff);
    crate::DebugLog(
        format!("Received manifest name size: {} bytes", MANIFEST_PATH_SIZE).as_str(),
        Color::Green,
    );

    // Receive file names
    let mut pathNameBuff = vec![0u8; PATH_SIZE as usize];
    let mut manifestNameBuff = vec![0u8; MANIFEST_PATH_SIZE as usize];

    tcp.read_exact(&mut pathNameBuff)?;
    crate::DebugLog(
        format!("Received file name: {}", String::from_utf8_lossy(&pathNameBuff)).as_str(),
        Color::Green,
    );
    tcp.read_exact(&mut manifestNameBuff)?;
    crate::DebugLog(
        format!("Received manifest name: {}", String::from_utf8_lossy(&manifestNameBuff)).as_str(),
        Color::Green,
    );

    let PATH_NAME = String::from_utf8(pathNameBuff)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let MANIFEST_NAME = String::from_utf8(manifestNameBuff)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Setting it to ignore file sync
    crate::IgnoreFileSync
        .lock()
        .unwrap()
        .push(PATH_NAME.to_string());

    // Receive manifest file
    let mut manifestFile = File::create(format!(
        "{}/{}.bixsync",
        crate::SYNC_FOLDER_LOCATION,
        MANIFEST_NAME
    ))?;
    crate::DebugLog("Manifest file created", Color::BrightWhite);

    receiveFile(&mut tcp, &mut manifestFile, MANIFEST_SIZE)?;
    crate::DebugLog("Manifest file received & saved", Color::Green);
    drop(manifestFile);
    let mut manifestFile = File::open(format!(
        "{}/{}.bixsync",
        crate::SYNC_FOLDER_LOCATION,
        MANIFEST_NAME
    ))?;

    // Comparing manifest
    match compareManifest(&mut manifestFile, PATH_NAME.to_string()) {
        // Receive sync file
        Ok(_) => {
            // Debug logs
            crate::DebugLog("Current manifest outdated... syncing", Color::Green);

            let mut syncFile = File::create(format!(
                "{}/{}.bixsync",
                crate::SYNC_FOLDER_LOCATION,
                PATH_NAME
            ))?;

            // Receiving sync file
            receiveFile(&mut tcp, &mut syncFile, SYNC_SIZE)?;
            crate::DebugLog("Sync file received & saved", Color::Green);

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

            // Debug logs
            crate::DebugLog("Cleanup completed", Color::Yellow);
            println!();
        }
        Err(E) => {
            // Debug logs
            crate::DebugLog("Failed to compare manifest", Color::Red);

            tcp.shutdown(Shutdown::Both)?;

            // Deleting files
            fs::remove_file(format!(
                "{}/{}.bixsync",
                crate::SYNC_FOLDER_LOCATION,
                MANIFEST_NAME
            ))?;

            // Debug logs
            crate::DebugLog(&format!("Failed to compare manifest: {E}"), Color::Red);
            println!();

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
        let _ = handleClient(STREAM).map_err(|E| {
            println!("Error while handeling client: {}", E);
        });
    }

    Ok(())
}
