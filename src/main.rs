#![allow(nonstandard_style)]

use bixsync::{peers, folderWatcher, sync};
use std::{io, thread};

// Main function
fn main() -> io::Result<()> {
    // Manifest checking
    bixsync::manifest::init()?;

    // Knowing Peers
    thread::spawn(peers::broadcaster::init);
    thread::spawn(peers::discoveryListener::init);

    // Monitoring changes in filesystem
    thread::spawn(folderWatcher::init);

    // Syncing
    let _ = sync::sendFile::init("/home/ishank/bixsync/Hello2.txt".to_string());

    loop {
        thread::park();
    }
}
