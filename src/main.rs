#![allow(nonstandard_style)]

use bixsync::{peers, folderWatcher};
use std::{io, thread};

// Main function
fn main() -> io::Result<()> {
    // Knowing Peers
    thread::spawn(peers::broadcaster::init);
    thread::spawn(peers::discoveryListener::init);
    thread::spawn(folderWatcher::init);

    loop {
        thread::park();
    }
}
