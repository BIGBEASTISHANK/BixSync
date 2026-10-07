#![allow(nonstandard_style)]

use bixsync::{folderWatcher, peers, sync};
use colored::Color;
use std::{io, thread};

// Main function
fn main() -> io::Result<()> {
    // Debug Logs
    bixsync::DebugLog(
        "Starting all required threads & initial check",
        Color::Yellow,
    );

    // Manifest checking
    bixsync::manifest::init()?;

    // Starting threads
    let BROADCASTER_THREAD = thread::spawn(peers::broadcaster::init); // Knowing Peers (Broadcaster)
    let DISCOVERY_LISTNER_THREAD = thread::spawn(peers::discoveryListener::init); // Knowing Peers (Listner)
    let FOLDER_WATCHER_THREAD = thread::spawn(folderWatcher::init); // Monitoring changes in filesystem
    let RECEIVE_FILE_THREAD = thread::spawn(sync::receiveFile::init); // Syncing

    // Log when thread is finished
    match BROADCASTER_THREAD.join() {
        Ok(_) => println!("[INFO] Broadcaster thread finished."),
        Err(_) => eprintln!("[ERROR] Broadcaster thread panicked."),
    }

    match DISCOVERY_LISTNER_THREAD.join() {
        Ok(_) => println!("[INFO] Discovery listener thread finished."),
        Err(_) => eprintln!("[ERROR] Discovery listener thread panicked."),
    }

    match FOLDER_WATCHER_THREAD.join() {
        Ok(_) => println!("[INFO] Folder watcher thread finished."),
        Err(_) => eprintln!("[ERROR] Folder watcher thread panicked."),
    }

    match RECEIVE_FILE_THREAD.join() {
        Ok(_) => println!("[INFO] Receive file thread finished."),
        Err(_) => eprintln!("[ERROR] Receive file thread panicked."),
    }

    loop {
        thread::park();
    }
}
