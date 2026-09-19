#![allow(nonstandard_style)]

use std::{io, thread};

use bixsync::peers;

// Main function
fn main() -> io::Result<()> {
    // Knowing Peers
    thread::spawn(peers::broadcaster::init);
    thread::spawn(peers::discoveryListner::init);

    loop {
        thread::park();
    }
}
