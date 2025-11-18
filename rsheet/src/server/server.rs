use rsheet_lib::connect::{Connection, Manager};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use super::connection::handle_connection;
use crate::spreadsheet::{run_worker_thread, Spreadsheet};

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error + Send + Sync>>
where
    M: Manager,
{
    let spreadsheet = Arc::new(Mutex::new(Spreadsheet::new()));
    let (sender, receiver) = std::sync::mpsc::channel::<(String, u64)>();
    let shutdown_flag = Arc::new(AtomicBool::new(false));

    // Spawn worker thread for dependency recalculation
    let worker_spreadsheet = Arc::clone(&spreadsheet);
    let worker_shutdown = Arc::clone(&shutdown_flag);
    let worker_sender = sender.clone();
    let worker_handle = thread::spawn(move || {
        run_worker_thread(worker_spreadsheet, receiver, worker_sender, worker_shutdown)
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

    // Wait for all connection threads to finish
    let worker_handle = join_handles.remove(0);
    for handle in join_handles {
        handle.join().unwrap().unwrap();
    }

    drop(sender);
    shutdown_flag.store(true, Ordering::Relaxed);
    worker_handle.join().unwrap().unwrap();

    Ok(())
}
