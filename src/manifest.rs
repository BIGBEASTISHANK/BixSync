use std::{
    fs,
    io::{self, Error, Read},
    path::Path,
};

pub fn init() -> io::Result<()> {
    // Debug message
    println!("Manifest checking started...");

    let PATH = Path::new(crate::MANIFEST_FILE);

    if !PATH.exists() {
        match fs::write(PATH, "[]") {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::new(e.kind(), "Failed to create manifest directory"));
            }
        }
    }

    Ok(())
}

pub fn manifestUpdate(path: &str) -> io::Result<()> {
    let mut manifestFile = fs::File::open(crate::MANIFEST_FILE)?;

    let mut manifestBuffer = String::new();
    manifestFile.read_to_string(&mut manifestBuffer)?;

    let mut manifest: Vec<crate::ManifestStructure> = serde_json::from_str(&manifestBuffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    drop(manifestFile);

    // Finding data and updating data
    if let Some(fileData) = manifest.iter_mut().find(|item| item.file == path) {
        fileData.updateId += 1;
    } else {
        // If not present add it
        manifest.push(crate::ManifestStructure {
            file: path.to_string(),
            updateId: 0,
        });
    }

    let JSON = serde_json::to_string_pretty(&manifest)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    fs::write(crate::MANIFEST_FILE, JSON)?;
    Ok(())
}