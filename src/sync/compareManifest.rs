use std::{
    fs::File,
    io::{self, Read, Result},
};

pub fn init(incomingManifestFile: &mut File, syncFileName: String) -> Result<()> {
    // Incoming manifest file
    let mut incomingManifestFileContent = String::new();
    incomingManifestFile.read_to_string(&mut incomingManifestFileContent)?;

    let INCOMING_JSON: Vec<crate::ManifestStructure> =
        serde_json::from_str(&incomingManifestFileContent)?;

    // Self Manifest
    let mut selfManifestFile = File::open(crate::MANIFEST_FILE)?;
    let mut selfManifestFileContent = String::new();
    selfManifestFile.read_to_string(&mut selfManifestFileContent)?;

    let SELF_JSON: Vec<crate::ManifestStructure> = serde_json::from_str(&selfManifestFileContent)?;

    for ITER in 0..INCOMING_JSON.len() {
        if INCOMING_JSON[ITER].file == syncFileName {
            if INCOMING_JSON[ITER].updateId > SELF_JSON[ITER].updateId {
                println!("Incoming manifest is newer than self manifest...");
                return Ok(());
            }
        }
    }

    // If not found, return error
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Incoming manifest is older than self manifest",
    ))
}
