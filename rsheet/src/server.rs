use rsheet_lib::connect::{Connection, Manager};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
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
    let shutdown_flag = Arc::new(AtomicBool::new(false));

    // Spawn the single worker thread
    let worker_spreadsheet_clone = Arc::clone(&spreadsheet);
    let worker_shutdown_flag = Arc::clone(&shutdown_flag);
    let worker_sender = sender.clone(); // Clone sender for worker thread to notify its own dependents
    let worker_handle = thread::spawn(move || {
        run_worker_thread(
            worker_spreadsheet_clone,
            receiver,
            worker_sender,
            worker_shutdown_flag,
        )
    });

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

    // Wait for all connection threads to finish first
    let worker_handle = join_handles.remove(0); // Remove worker handle
    for handle in join_handles {
        handle.join().unwrap().unwrap();
    }

    // Now drop the original sender - this signals that no more messages will come from connection threads
    drop(sender);

    // Signal the worker thread to shutdown
    shutdown_flag.store(true, Ordering::Relaxed);

    // Wait for worker thread to finish processing
    worker_handle.join().unwrap().unwrap();

    Ok(())
}
