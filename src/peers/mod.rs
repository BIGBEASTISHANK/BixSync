use std::fs;

pub mod broadcaster;
pub mod discoveryListner;

// Load peers
pub fn LoadPears() -> Vec<String> {
    if let Ok(TEXT) = fs::read_to_string(crate::PEERS_FILE) {
        serde_json::from_str(&TEXT).unwrap_or_default()
    } else {
        vec![]
    }
}

// Save peers
pub fn SavePeers(PEERS: &[String]) {
    if let Ok(JSON) = serde_json::to_string_pretty(PEERS) {
        let _ = fs::write(crate::PEERS_FILE, JSON);
    }
}