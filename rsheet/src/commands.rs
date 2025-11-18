use rsheet_lib::cell_expr::{CellArgument, CellExpr};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use rsheet_lib::command::Command;
use rsheet_lib::replies::Reply;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::parsing::parse_cell_range;
use crate::spreadsheet::Spreadsheet;

pub fn handle_get_command(
    spreadsheet: &Arc<Mutex<Spreadsheet>>,
    cell_identifier: rsheet_lib::command::CellIdentifier,
) -> Reply {
    let spreadsheet_guard = spreadsheet.lock().unwrap();
    let cell_id_string = format!(
        "{}{}",
        column_number_to_name(cell_identifier.col),
        cell_identifier.row + 1
    );
    let value = spreadsheet_guard.get_cell(&cell_id_string);
    drop(spreadsheet_guard); // Release lock early

    if value == CellValue::Error("DEPENDENCY_ERROR_MARKER".to_string()) {
        Reply::Error("getting the value of a cell that depends on another cell with an error is not allowed.".to_string())
    } else {
        Reply::Value(cell_id_string, value)
    }
}

pub fn handle_set_command(
    spreadsheet: &Arc<Mutex<Spreadsheet>>,
    sender: &std::sync::mpsc::Sender<(String, u64)>,
    cell_identifier: rsheet_lib::command::CellIdentifier,
    cell_expr: String,
) {
    let mut spreadsheet_guard = spreadsheet.lock().unwrap();
    let current_version = spreadsheet_guard.get_next_version();
    let new_cell_expr = CellExpr::new(&cell_expr);

    let mut variables_map: HashMap<String, CellArgument> = HashMap::new();
    let mut dependencies: HashSet<String> = HashSet::new();

    for var_name in new_cell_expr.find_variable_names() {
        dependencies.insert(var_name.clone()); // Record dependency

        let (start_id, end_id) = match parse_cell_range(&var_name) {
            Ok(ids) => ids,
            Err(e) => {
                variables_map.insert(var_name, CellArgument::Value(CellValue::Error(e)));
                continue;
            }
        };

        if start_id == end_id {
            let cell_id_string = format!(
                "{}{}",
                column_number_to_name(start_id.col),
                start_id.row + 1
            );
            let value = spreadsheet_guard.get_cell(&cell_id_string);
            variables_map.insert(var_name, CellArgument::Value(value));
        } else {
            let mut matrix = Vec::new();
            for row in start_id.row..=end_id.row {
                let mut row_vec = Vec::new();
                for col in start_id.col..=end_id.col {
                    let cell_id_string = format!("{}{}", column_number_to_name(col), row + 1);
                    let value = spreadsheet_guard.get_cell(&cell_id_string);
                    row_vec.push(value);
                }
                matrix.push(row_vec);
            }

            if start_id.col == end_id.col || start_id.row == end_id.row {
                let vector = matrix.into_iter().flatten().collect();
                variables_map.insert(var_name, CellArgument::Vector(vector));
            } else {
                variables_map.insert(var_name, CellArgument::Matrix(matrix));
            }
        }
    }

    let evaluated_value = new_cell_expr.evaluate(&variables_map);

    let cell_id_string = format!(
        "{}{}",
        column_number_to_name(cell_identifier.col),
        cell_identifier.row + 1
    );
    spreadsheet_guard.set_cell(
        cell_id_string.clone(), // Clone for insertion
        cell_expr.clone(),      // Store the raw expression string
        dependencies,
        evaluated_value,
        current_version,
    );

    // Notify dependents to re-evaluate
    // For Stage 4, only direct dependents (one-level deep)
    let mut dependents_to_notify: HashSet<String> = HashSet::new();
    if let Some(entry) = spreadsheet_guard.get_cell_entry(&cell_id_string) {
        dependents_to_notify.extend(entry.dependents.iter().cloned());
    }
    drop(spreadsheet_guard); // Release lock before sending messages

    for dependent_id in dependents_to_notify {
        sender.send((dependent_id, current_version)).unwrap();
    }
}

pub fn handle_command(
    spreadsheet: &Arc<Mutex<Spreadsheet>>,
    sender: &std::sync::mpsc::Sender<(String, u64)>,
    command: Command,
) -> Option<Reply> {
    match command {
        Command::Get { cell_identifier } => Some(handle_get_command(spreadsheet, cell_identifier)),
        Command::Set {
            cell_identifier,
            cell_expr,
        } => {
            handle_set_command(spreadsheet, sender, cell_identifier, cell_expr);
            None // Set commands have no output
        }
    }
}
