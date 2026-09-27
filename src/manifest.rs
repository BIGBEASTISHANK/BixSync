use std::{fs, io::{self, Error}, path::Path};

pub fn init() -> io::Result<()> {
    // Debug message
    println!("Manifest checking started...");

    let PATH = Path::new(crate::MANIFEST_FILE);

    if !PATH.exists() {
        match fs::write(PATH, "{}") {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::new(e.kind(), "Failed to create manifest directory"));
            }
        }
    }

    Ok(())
}

pub fn fileUpdated(path: &str) -> io::Result<()> {
    Ok(())
}

pub fn fileAdded(path: &str) -> io::Result<()> {
    Ok(())
}
