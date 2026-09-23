# BixSync

It's a file sync system, that syncs a folder to all the devices connected to the same network.

> [!NOTE]
> Just a fun and caotic project, it will break if multiple user will try to edit same file at the same time.

## Tech

- Rust
- Notify
- Serde
- Serde JSON

## Working Explain

> [!NOTE]
> This working explanation is in simple words, their is a lot more going on in the code.

- **FolderWatcher()**: This function is used to watch changes in filesystem in set directory which we will be using to sync. It watches changes such as _DELETE_, _UPDATE_ & _CREATE_ and perform action accordingly.
- **RequestInitialSync()**: The use of this function is to request initial sync from all the peers. If a file is missing in the local folder it will download it from other peers and sync it to our own device.
- **SyncFiles()**: This function is used to sync files to all the peers. Basically sending files to all other peers. It's a sender.
- **TcpSyncServer()**: This function is also used to sync files but it recieves files insted of sending lik _SyncFiles()_. It's a reciever.
- **Broadcaster()**: This function broadcast message (made specifically for bixsync) to all devices connected to the same network.
- **DiscoveryListener()**: This function is used to listen for broadcasted messages and if they are from bixsync it will add them to the list of peers.

### Minor functions

- **LoadPears()**: This function is used to load peers from file.
- **SavePeers()**: This function is used to save peers to file.
- **GenerateManifest()**: This function is used to generate manifest of files (which files exist in the sync folder) in the local folder.
- **SendMessage()**: This function is used to send message to other peers.
- **ReceiveMessage()**: This function is used to receive message from other peers.

## How to use

1. Clone the repo.
2. In `lib.rs` file, change the `SYNC_FOLDER_LOCATION` to the path of the folder you want to sync.
3. Run `cargo run` in the terminal.
4. Now you can edit files in the sync folder and they will be synced to all the devices connected to the same network running bixsync.


## TEAM : DEVSORA

[By Augustus (HB Singh)](https://github.com/BYAUGUSTUS/)

[BIGBEASTISHANK (Pranjal)](https://bigbeastishank.com/)

[Ankit](https://github.com//)

[Utkarsh](https://github.com//)
