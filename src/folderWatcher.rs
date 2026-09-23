use notify::{Event, RecursiveMode, Result, Watcher};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
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
    // Debug message
    println!("FolderWatcher started");

    // Folder checker / creator
    let PATH = Path::new(crate::SYNC_FOLDER_LOCATION);

    if !PATH.exists() {
        fs::create_dir_all(PATH)?;
    }

    // Create a channel to receive the events.
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(PATH, RecursiveMode::NonRecursive)?;

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
                println!("Watch error: {:?}", e);
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
                    if EVENT.write {
                        println!("Edit event: {:?}", EVENT_PATH);
                    } else if EVENT.create {
                        println!("Create event: {:?}", EVENT_PATH);
                    }
                } else if EVENT.remove {
                    // Delete
                    println!("Delete event: {:?}", EVENT_PATH);
                }
            }
        }
    }

    Ok(())
}
