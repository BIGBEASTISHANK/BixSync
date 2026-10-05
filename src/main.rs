#![allow(nonstandard_style)]

use bixsync::{folderWatcher, peers, sync};
use std::{io, thread};

// Main function
fn main() -> io::Result<()> {
    // Manifest checking
    bixsync::manifest::init()?;

    // Knowing Peers
    let BROADCASTER_THREAD = thread::spawn(peers::broadcaster::init);
    let DISCOVERY_LISTNER_THREAD = thread::spawn(peers::discoveryListener::init);

    // Monitoring changes in filesystem
    let FOLDER_WATCHER_THREAD = thread::spawn(folderWatcher::init);

    // Syncing
    let RECEIVE_FILE_THREAD = thread::spawn(sync::receiveFile::init);

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
