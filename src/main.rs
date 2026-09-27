#![allow(nonstandard_style)]

use bixsync::{peers, folderWatcher};
use std::{io, thread};

// Main function
fn main() -> io::Result<()> {
    // Manifest checking
    bixsync::manifest::init()?;
    bixsync::manifest::fileUpdated("test.txt")?;

    // Knowing Peers
    thread::spawn(peers::broadcaster::init);
    thread::spawn(peers::discoveryListener::init);

    // Monitoring changes in filesystem
    thread::spawn(folderWatcher::init);

    loop {
        thread::park();
    }
}
