use rsheet_lib::connect::{Connection, Manager};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::connection::handle_connection;
use crate::spreadsheet::Spreadsheet;
use crate::worker::run_worker_thread;

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error + Send + Sync>>
where
    M: Manager,
{
    let spreadsheet = Arc::new(Mutex::new(Spreadsheet::new()));
    let (sender, receiver) = std::sync::mpsc::channel::<(String, u64)>();

    // Spawn the single worker thread
    let worker_spreadsheet_clone = Arc::clone(&spreadsheet);
    let worker_sender = sender.clone(); // Clone sender for worker thread to notify its own dependents
    let worker_handle =
        thread::spawn(move || run_worker_thread(worker_spreadsheet_clone, receiver, worker_sender));

    let mut join_handles = Vec::new();
    join_handles.push(worker_handle);

    loop {
        let connection = manager.accept_new_connection();
        match connection {
            Connection::NewConnection { reader, writer } => {
                let spreadsheet_clone = Arc::clone(&spreadsheet);
                let sender_clone = sender.clone();

                let handle = thread::spawn(move || {
                    handle_connection(reader, writer, spreadsheet_clone, sender_clone)
                });
                join_handles.push(handle);
            }
            Connection::NoMoreConnections => {
                break;
            }
        }
    }

    drop(sender); // Explicitly drop sender to allow worker thread to terminate
    for handle in join_handles {
        handle.join().unwrap().unwrap();
    }

    Ok(())
}
