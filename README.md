# BixSync

It's a file sync system, that syncs a folder to all the devices connected to the same network.

## Working / Pipeline
![WorkingPipeline](./BixSync%20Pipeline.drawio.png)

## How to use

1. Clone the repo.
2. In `lib.rs` file, change the `SYNC_FOLDER_LOCATION` to the path of the folder you want to sync.
3. Run `cargo run` in the terminal.
4. Now you can edit files in the sync folder and they will be synced to all the devices connected to the same network running bixsync.