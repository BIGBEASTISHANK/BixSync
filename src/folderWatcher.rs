use colored::Color;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
struct PendingEvent {
    create: bool,
    remove: bool,
    write: bool,
    last_event: Instant,
}

pub fn init() -> Result<()> {
    // Folder checker / creator
    let PATH = Path::new(crate::SYNC_FOLDER_LOCATION);

    if !PATH.exists() {
        fs::create_dir_all(PATH)?;
    }

    // Create a channel to receive the events.
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(PATH, RecursiveMode::NonRecursive)?;

    // Debug message
    println!("FolderWatcher started");
    crate::ThreadReady();

    // Pending events
    let mut pending: HashMap<PathBuf, PendingEvent> = HashMap::new();

    // Wait for DEBOUNCE after last event before processing
    let DEBOUNCE = Duration::from_millis(100);

    loop {
        let NOW = Instant::now();

        let WAIT = pending
            .values()
            .map(|event| {
                DEBOUNCE
                    .checked_sub(NOW.duration_since(event.last_event))
                    .unwrap_or(Duration::ZERO)
            })
            .min()
            .unwrap_or(Duration::from_secs(60));

        // Calculate how long to wait for the next event or pending-event timeout
        match rx.recv_timeout(WAIT) {
            Ok(Ok(EVENT)) => {
                if EVENT.paths.is_empty() {
                    continue;
                }

                for EVENT_PATH in EVENT.paths {
                    // Ignore temporary files
                    if EVENT_PATH
                        .file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| {
                            name.starts_with(".goutputstream-")
                                || name.ends_with('~')
                                || name.ends_with(".swp")
                                || name.ends_with(".swo")
                                || name.ends_with(".tmp")
                                || name.ends_with(".bixsync")
                        })
                    {
                        continue;
                    }

                    let ENTRY = pending
                        .entry(EVENT_PATH.clone())
                        .or_insert_with(|| PendingEvent {
                            create: false,
                            remove: false,
                            write: false,
                            last_event: Instant::now(),
                        });

                    ENTRY.last_event = Instant::now();

                    match EVENT.kind {
                        // Create
                        notify::EventKind::Create(_) => {
                            ENTRY.create = true;
                        }

                        // Delete
                        notify::EventKind::Remove(_) => {
                            ENTRY.remove = true;
                        }

                        // Rename / move
                        notify::EventKind::Modify(notify::event::ModifyKind::Name(_)) => {
                            if !EVENT_PATH.exists() {
                                ENTRY.remove = true;
                            } else {
                                ENTRY.write = true;
                            }
                        }

                        // Write / edit
                        notify::EventKind::Modify(notify::event::ModifyKind::Data(_)) => {
                            ENTRY.write = true;
                        }

                        // Other
                        _ => {}
                    }
                }
            }

            // Watcher reported an error
            Ok(Err(e)) => {
                crate::DebugLog(&format!("Watch error: {:?}", e), Color::Red);
            }

            // No event arrived before the timeout
            Err(mpsc::RecvTimeoutError::Timeout) => {}

            // Event channel was disconnected
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }

        // Process pending events
        let NOW = Instant::now();

        let EXPIRED: Vec<PathBuf> = pending
            .iter()
            .filter(|(_, EVENT)| NOW.duration_since(EVENT.last_event) >= DEBOUNCE)
            .map(|(EVENT_PATH, _)| EVENT_PATH.clone())
            .collect();

        // Find events that have been quiet for DEBOUNCE
        for EVENT_PATH in EXPIRED {
            if let Some(EVENT) = pending.remove(&EVENT_PATH) {
                if EVENT_PATH.exists() {
                    // Edit / create
                    if EVENT.write || EVENT.create {
                        // Check if file should be ignored
                        let SHOULD_IGNORE = {
                            let mut IGNORE = crate::IgnoreFileSync.lock().unwrap();

                            let EVENT_PATH = EVENT_PATH
                                .to_str()
                                .unwrap()
                                .replace(crate::SYNC_FOLDER_LOCATION, "")
                                .trim_start_matches('/')
                                .to_string();

                            if let Some(INDEX) = IGNORE.iter().position(|path| path == &EVENT_PATH)
                            {
                                IGNORE.remove(INDEX);
                                true
                            } else {
                                false
                            }
                        };

                        if SHOULD_IGNORE {
                            continue;
                        }

                        // Debug Logs
                        println!();
                        crate::DebugLog("Event Captured. Processing...", Color::Yellow);
                        println!("Update/Create event: {:?}", EVENT_PATH);

                        crate::manifest::manifestUpdate(
                            &EVENT_PATH
                                .to_str()
                                .unwrap()
                                .replace(crate::SYNC_FOLDER_LOCATION, "")[1..]
                                .to_string(),
                            None,
                        )?;

                        // Open peers list
                        let PEERS = crate::peers::LoadPears();

                        for iter in PEERS {
                            let PATH = EVENT_PATH.to_str().unwrap().to_string();
                            let PEER = iter.to_string();

                            println!("Sending file to {}", PEER);

                            thread::spawn(move || {
                                match crate::sync::sendFile::init(PATH, PEER.clone()) {
                                    Ok(_) => {}
                                    Err(E) => {
                                        crate::DebugLog(
                                            &format!("Failed to send file to {}: {}", PEER, E),
                                            Color::Red,
                                        );
                                    }
                                }
                            });
                        }

                        // Debug Logs
                        crate::DebugLog("Initiated sync with all online clients", Color::Yellow);
                        println!();
                    }
                } else if EVENT.remove {
                    // Debug Logs
                    println!();
                    crate::DebugLog("Event Captured. Processing...", Color::Yellow);
                    println!("Delete event: {:?}", EVENT_PATH);
                }
            }
        }
    }

    Ok(())
}
