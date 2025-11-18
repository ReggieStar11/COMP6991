use rsheet_lib::cell_expr::CellExpr;
use std::error::Error;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::spreadsheet::Spreadsheet;
use super::variables::collect_variables;

pub fn run_worker_thread(
    spreadsheet: Arc<Mutex<Spreadsheet>>,
    receiver: std::sync::mpsc::Receiver<(String, u64)>,
    sender: std::sync::mpsc::Sender<(String, u64)>,
    shutdown_flag: Arc<std::sync::atomic::AtomicBool>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        let msg = receiver.recv_timeout(Duration::from_millis(100));
        match msg {
            Ok((cell_id_to_recalculate, _triggering_version)) => {
                let mut spreadsheet_guard = spreadsheet.lock().unwrap();

                if let Some(entry) = spreadsheet_guard.get_cell_entry(&cell_id_to_recalculate) {
                    let expr_string = entry.expr_string.clone();
                    let dependencies = entry.dependencies.clone();

                    // Re-evaluate cell with current dependency values
                    let new_cell_expr = CellExpr::new(&expr_string);
                    let temp_spreadsheet_values =
                        collect_variables(&new_cell_expr, &*spreadsheet_guard);
                    let evaluated_value = new_cell_expr.evaluate(&temp_spreadsheet_values);

                    // Each recalculation gets a new version number
                    let new_version = spreadsheet_guard.get_next_version();

                    spreadsheet_guard.set_cell(
                        cell_id_to_recalculate.clone(),
                        expr_string,
                        dependencies,
                        evaluated_value,
                        new_version,
                    );

                    // Recursively notify dependents (Stage 5: multi-layered dependencies)
                    let dependents_to_notify: Vec<String> = spreadsheet_guard
                        .get_cell_entry(&cell_id_to_recalculate)
                        .map(|e| e.dependents.iter().cloned().collect())
                        .unwrap_or_default();
                    drop(spreadsheet_guard);

                    for dependent_id in dependents_to_notify {
                        sender.send((dependent_id, new_version)).unwrap();
                    }
                } else {
                    drop(spreadsheet_guard);
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if shutdown_flag.load(Ordering::Relaxed) {
                    // Drain any remaining messages before exiting
                    while receiver.try_recv().is_ok() {}
                    break;
                }
                continue;
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}
