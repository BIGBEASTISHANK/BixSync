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
    thread::spawn(sync::receiveFile::init);

    loop {
        thread::park();
    }
}
