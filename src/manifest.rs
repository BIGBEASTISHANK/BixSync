use std::{
    fs::{self, File},
    io::{self, Error, Read},
    path::Path,
};

pub fn init() -> io::Result<()> {
    // Debug message
    println!("Manifest checking started");

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

pub fn manifestUpdate(PATH: &str, updateId: Option<i32>) -> io::Result<()> {
    let mut manifestFile = fs::File::open(crate::MANIFEST_FILE)?;

    let mut manifestBuffer = String::new();
    manifestFile.read_to_string(&mut manifestBuffer)?;

    let mut manifest: Vec<crate::ManifestStructure> = serde_json::from_str(&manifestBuffer)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    drop(manifestFile);

    // Finding data and updating data
    if let Some(fileData) = manifest
        .iter_mut()
        .find(|item| item.file == PATH.replace(crate::SYNC_FOLDER_LOCATION, "")[1..])
    {
        fileData.updateId += 1;
    } else {
        // updateId initialize
        let INITIALIZE_UPDATE_ID = match updateId {
            Some(ID) => ID,
            None => 0,
        };

        // If not present add it
        manifest.push(crate::ManifestStructure {
            file: PATH
                .to_string()
                .replace(crate::SYNC_FOLDER_LOCATION, "")
                .to_string(),
            updateId: INITIALIZE_UPDATE_ID,
        });
    }

    let JSON = serde_json::to_string_pretty(&manifest)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    fs::write(crate::MANIFEST_FILE, JSON)?;
    Ok(())
}

// Manifest comparision
pub fn compareManifest(incomingManifestFile: &mut File, syncFileName: String) -> io::Result<()> {
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
    match SELF_JSON.iter().find(|&x| x.file == syncFileName) {
        Some(ENTRY) => {
            for ITER in 0..INCOMING_JSON.len() {
                if INCOMING_JSON[ITER].file == syncFileName {
                    if INCOMING_JSON[ITER].updateId < ENTRY.updateId {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Incoming manifest is older than self manifest",
                        ));
                    }
                }
            }
        }
        None => {
            for ITER in 0..INCOMING_JSON.len() {
                if INCOMING_JSON[ITER].file == syncFileName {
                    crate::manifest::manifestUpdate(
                        &syncFileName,
                        Some(INCOMING_JSON[ITER].updateId),
                    )?;
                }
            }
        }
    }

    Ok(())
}
