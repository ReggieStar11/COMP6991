use rsheet_lib::cell_expr::CellExpr;
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::command::Command;
use rsheet_lib::replies::Reply;
use std::collections::HashSet;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::parsing::format_cell_identifier;
use crate::spreadsheet::{collect_variables, Spreadsheet};

pub fn handle_get_command(
    spreadsheet: &Arc<Mutex<Spreadsheet>>,
    cell_identifier: rsheet_lib::command::CellIdentifier,
) -> Reply {
    let spreadsheet_guard = spreadsheet.lock().unwrap();
    let cell_id_string = format_cell_identifier(&cell_identifier);
    let value = spreadsheet_guard.get_cell(&cell_id_string);

    if value == CellValue::Error("DEPENDENCY_ERROR_MARKER".to_string()) {
        Reply::Error("getting the value of a cell that depends on another cell with an error is not allowed.".to_string())
    } else {
        Reply::Value(cell_id_string, value)
    }
}

pub fn handle_set_command(
    spreadsheet: &Arc<Mutex<Spreadsheet>>,
    sender: &Sender<(String, u64)>,
    cell_identifier: rsheet_lib::command::CellIdentifier,
    cell_expr: String,
) {
    let new_cell_expr = CellExpr::new(&cell_expr);
    let dependencies: HashSet<String> = new_cell_expr.find_variable_names().into_iter().collect();
    let cell_id_string = format_cell_identifier(&cell_identifier);

    // Get variables quickly, then release mutex before evaluating
    // This way if evaluation blocks (like with sleep_then), other threads can still access the spreadsheet
    let variables_map = {
        let spreadsheet_guard = spreadsheet.lock().unwrap();
        collect_variables(&new_cell_expr, &spreadsheet_guard)
    };
    let evaluated_value = new_cell_expr.evaluate(&variables_map);

    // Lock again to update the spreadsheet
    let mut spreadsheet_guard = spreadsheet.lock().unwrap();
    let current_version = spreadsheet_guard.get_next_version();
    spreadsheet_guard.set_cell(
        cell_id_string.clone(),
        cell_expr.clone(),
        dependencies,
        evaluated_value,
        current_version,
    );

    let dependents_to_notify: Vec<String> = spreadsheet_guard
        .get_cell_entry(&cell_id_string)
        .map(|e| e.dependents.iter().cloned().collect())
        .unwrap_or_default();

    for dependent_id in dependents_to_notify {
        sender.send((dependent_id, current_version)).unwrap();
    }
}

pub fn handle_command(
    spreadsheet: &Arc<Mutex<Spreadsheet>>,
    sender: &Sender<(String, u64)>,
    command: Command,
) -> Option<Reply> {
    match command {
        Command::Get { cell_identifier } => Some(handle_get_command(spreadsheet, cell_identifier)),
        Command::Set {
            cell_identifier,
            cell_expr,
        } => {
            handle_set_command(spreadsheet, sender, cell_identifier, cell_expr);
            None
        }
    }
}
