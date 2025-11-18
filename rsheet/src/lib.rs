use log::info;
use rsheet_lib::cell_expr::{CellArgument, CellExpr, CellExprEvalError};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

// Represents the state of a single cell in the spreadsheet.
struct CellEntry {
    expr_string: String,
    dependencies: HashSet<String>, // Names of cells this cell directly depends on
    dependents: HashSet<String>,   // Names of cells that depend on this cell
    value: CellValue,
    version: u64, // For handling temporal ordering of updates
}

// A simple struct to hold our spreadsheet data.
struct Spreadsheet {
    cells: HashMap<String, CellEntry>,
    next_version: u64,
}

impl Spreadsheet {
    fn new() -> Self {
        Spreadsheet {
            cells: HashMap::new(),
            next_version: 0,
        }
    }

    fn get_cell(&self, cell_identifier: &str) -> CellValue {
        self.cells
            .get(cell_identifier)
            .map(|entry| entry.value.clone())
            .unwrap_or(CellValue::None)
    }

    fn set_cell(
        &mut self,
        cell_identifier: String,
        expr_string: String,
        new_dependencies: HashSet<String>,
        value: Result<CellValue, rsheet_lib::cell_expr::CellExprEvalError>,
        version: u64,
    ) {
        let cell_value = match value {
            Ok(cell_value) => cell_value,
            Err(e) => match e {
                rsheet_lib::cell_expr::CellExprEvalError::VariableDependsOnError => {
                    CellValue::Error("DEPENDENCY_ERROR_MARKER".to_string())
                }
            },
        };

        // Only update if the new version is newer than the existing one, or if no entry exists.
        let current_version = self
            .cells
            .get(&cell_identifier)
            .map(|entry| entry.version)
            .unwrap_or(0);
        if version >= current_version {
            let mut old_dependencies = HashSet::new();
            if let Some(old_entry) = self.cells.get(&cell_identifier) {
                old_dependencies = old_entry.dependencies.clone();
            }

            // Remove this cell from the dependents of its old dependencies
            for dep in &old_dependencies {
                if let Some(dep_entry) = self.cells.get_mut(dep) {
                    dep_entry.dependents.remove(&cell_identifier);
                }
            }

            // Insert the new cell entry
            self.cells.insert(
                cell_identifier.clone(),
                CellEntry {
                    expr_string,
                    dependencies: new_dependencies.clone(),
                    dependents: HashSet::new(), // This will be populated by its dependencies
                    value: cell_value,
                    version,
                },
            );

            // Add this cell to the dependents of its new dependencies
            for dep in &new_dependencies {
                if let Some(dep_entry) = self.cells.get_mut(dep) {
                    dep_entry.dependents.insert(cell_identifier.clone());
                }
            }
        }
    }

    // Helper to get the next version number atomically
    fn get_next_version(&mut self) -> u64 {
        let current_version = self.next_version;
        self.next_version += 1;
        current_version
    }
}

// Helper function to parse a column name (e.g., "A", "AA") into a 0-indexed number.
fn column_name_to_number(column_name: &str) -> Result<u32, String> {
    let mut col_num = 0;
    for char_code in column_name.to_uppercase().chars().map(|c| c as u32) {
        if !(('A' as u32)..=('Z' as u32)).contains(&char_code) {
            return Err(format!("Invalid character in column name: {}", column_name));
        }
        col_num = col_num * 26 + (char_code - 'A' as u32 + 1);
    }
    // Convert from 1-indexed to 0-indexed
    Ok(col_num - 1)
}

// Helper function to parse a cell identifier string (e.g., "A1") into a CellIdentifier struct.
fn parse_cell_identifier_string(s: &str) -> Result<CellIdentifier, String> {
    let mut col_chars = String::new();
    let mut row_chars = String::new();
    for c in s.chars() {
        if c.is_alphabetic() {
            col_chars.push(c);
        } else if c.is_ascii_digit() {
            row_chars.push(c);
        } else {
            return Err(format!("Invalid character in cell identifier: {}", s));
        }
    }

    if col_chars.is_empty() || row_chars.is_empty() {
        return Err(format!("Invalid cell identifier format: {}", s));
    }

    let col = column_name_to_number(&col_chars)?;
    let row = row_chars.parse::<u32>().map_err(|e| e.to_string())?;

    // Convert 1-indexed row to 0-indexed
    Ok(CellIdentifier { col, row: row - 1 })
}

// Helper function to parse a cell range string (e.g., "A1", "A1_B3") into start and end CellIdentifier structs.
// If it's a single cell, start and end will be the same.
fn parse_cell_range(s: &str) -> Result<(CellIdentifier, CellIdentifier), String> {
    let parts: Vec<&str> = s.split('_').collect();

    match parts.len() {
        1 => {
            // Single cell reference
            let cell_id = parse_cell_identifier_string(s)?;
            Ok((cell_id, cell_id))
        }
        2 => {
            // Cell range reference (e.g., "A1_B3")
            let start_id = parse_cell_identifier_string(parts[0])?;
            let end_id = parse_cell_identifier_string(parts[1])?;
            Ok((start_id, end_id))
        }
        _ => Err(format!("Invalid cell range format: {}", s)),
    }
}

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let spreadsheet = Arc::new(Mutex::new(Spreadsheet::new()));
    let (sender, receiver) = std::sync::mpsc::channel::<(String, u64)>(); // Channel for worker thread

    // Spawn the single worker thread
    let worker_spreadsheet_clone = Arc::clone(&spreadsheet);
    let worker_handle = thread::spawn(move || {
        for (cell_id_to_recalculate, triggering_version) in receiver {
            let mut spreadsheet_guard = worker_spreadsheet_clone.lock().unwrap();

            if let Some(entry) = spreadsheet_guard.cells.get(&cell_id_to_recalculate) {
                // Check if this recalculation is based on an older version
                if triggering_version < entry.version {
                    continue; // Skip if a newer version has already updated this cell
                }

                let expr_string = entry.expr_string.clone();
                let dependencies = entry.dependencies.clone();

                // Re-evaluate using current values of dependencies
                let new_cell_expr = CellExpr::new(&expr_string);
                let mut variables_map: HashMap<String, CellArgument> = HashMap::new();

                // Temporarily release the lock to gather dependencies if needed, then re-acquire.
                // For Stage 4, this is fine because we're only reading, not triggering more re-evaluations.
                let mut temp_spreadsheet_values = HashMap::new();
                for var_name in new_cell_expr.find_variable_names() {
                    let (start_id, end_id) = match parse_cell_range(&var_name) {
                        Ok(ids) => ids,
                        Err(e) => {
                            variables_map
                                .insert(var_name, CellArgument::Value(CellValue::Error(e)));
                            continue;
                        }
                    };

                    if start_id == end_id {
                        let cell_id_string_dep = format!(
                            "{}{}",
                            column_number_to_name(start_id.col),
                            start_id.row + 1
                        );
                        let value = spreadsheet_guard.get_cell(&cell_id_string_dep);
                        temp_spreadsheet_values.insert(var_name, CellArgument::Value(value));
                    } else {
                        let mut matrix = Vec::new();
                        for row in start_id.row..=end_id.row {
                            let mut row_vec = Vec::new();
                            for col in start_id.col..=end_id.col {
                                let cell_id_string_dep =
                                    format!("{}{}", column_number_to_name(col), row + 1);
                                let value = spreadsheet_guard.get_cell(&cell_id_string_dep);
                                row_vec.push(value);
                            }
                            matrix.push(row_vec);
                        }

                        if start_id.col == end_id.col || start_id.row == end_id.row {
                            let vector = matrix.into_iter().flatten().collect();
                            temp_spreadsheet_values.insert(var_name, CellArgument::Vector(vector));
                        } else {
                            temp_spreadsheet_values.insert(var_name, CellArgument::Matrix(matrix));
                        }
                    }
                }

                let evaluated_value = new_cell_expr.evaluate(&temp_spreadsheet_values);

                spreadsheet_guard.set_cell(
                    cell_id_to_recalculate.clone(),
                    expr_string,
                    dependencies,
                    evaluated_value,
                    triggering_version,
                );
            }
        }
        Ok::<(), Box<dyn Error + Send + Sync>>(())
    });

    let mut join_handles = Vec::new();
    join_handles.push(worker_handle);

    loop {
        let connection = manager.accept_new_connection();
        match connection {
            Connection::NewConnection { reader, writer } => {
                let spreadsheet_clone = Arc::clone(&spreadsheet);
                let sender_clone = sender.clone(); // Clone sender for each thread

                let handle = thread::spawn(move || {
                    let (mut recv, mut send) = (reader, writer);
                    loop {
                        info!("Just got message");
                        match recv.read_message() {
                            ReadMessageResult::Message(msg) => {
                                let command_result = msg.parse::<Command>();

                                match command_result {
                                    Ok(command) => {
                                        let mut spreadsheet_guard =
                                            spreadsheet_clone.lock().unwrap();
                                        match command {
                                            Command::Get { cell_identifier } => {
                                                let cell_id_string = format!(
                                                    "{}{}",
                                                    column_number_to_name(cell_identifier.col),
                                                    cell_identifier.row + 1
                                                );
                                                let value =
                                                    spreadsheet_guard.get_cell(&cell_id_string);
                                                drop(spreadsheet_guard); // Release lock early
                                                let reply = if value
                                                    == CellValue::Error(
                                                        "DEPENDENCY_ERROR_MARKER".to_string(),
                                                    ) {
                                                    Reply::Error("getting the value of a cell that depends on another cell with an error is not allowed.".to_string())
                                                } else {
                                                    Reply::Value(cell_id_string, value)
                                                };
                                                match send.write_message(reply) {
                                                    WriteMessageResult::Ok => { /* Message successfully sent, continue. */
                                                    }
                                                    WriteMessageResult::ConnectionClosed => {
                                                        break;
                                                    }
                                                    WriteMessageResult::Err(e) => {
                                                        return Err(e.into());
                                                    }
                                                }
                                            }
                                            Command::Set {
                                                cell_identifier,
                                                cell_expr,
                                            } => {
                                                let current_version =
                                                    spreadsheet_guard.get_next_version();
                                                let new_cell_expr = CellExpr::new(&cell_expr);

                                                let mut variables_map: HashMap<
                                                    String,
                                                    CellArgument,
                                                > = HashMap::new();

                                                let mut dependencies: HashSet<String> =
                                                    HashSet::new();

                                                for var_name in new_cell_expr.find_variable_names()
                                                {
                                                    dependencies.insert(var_name.clone()); // Record dependency

                                                    let (start_id, end_id) =
                                                        match parse_cell_range(&var_name) {
                                                            Ok(ids) => ids,
                                                            Err(e) => {
                                                                variables_map.insert(
                                                                    var_name,
                                                                    CellArgument::Value(
                                                                        CellValue::Error(e),
                                                                    ),
                                                                );
                                                                continue;
                                                            }
                                                        };

                                                    if start_id == end_id {
                                                        let cell_id_string = format!(
                                                            "{}{}",
                                                            column_number_to_name(start_id.col),
                                                            start_id.row + 1
                                                        );
                                                        let value = spreadsheet_guard
                                                            .get_cell(&cell_id_string);
                                                        variables_map.insert(
                                                            var_name,
                                                            CellArgument::Value(value),
                                                        );
                                                    } else {
                                                        let mut matrix = Vec::new();
                                                        for row in start_id.row..=end_id.row {
                                                            let mut row_vec = Vec::new();
                                                            for col in start_id.col..=end_id.col {
                                                                let cell_id_string = format!(
                                                                    "{}{}",
                                                                    column_number_to_name(col),
                                                                    row + 1
                                                                );
                                                                let value = spreadsheet_guard
                                                                    .get_cell(&cell_id_string);
                                                                row_vec.push(value);
                                                            }
                                                            matrix.push(row_vec);
                                                        }

                                                        if start_id.col == end_id.col
                                                            || start_id.row == end_id.row
                                                        {
                                                            let vector = matrix
                                                                .into_iter()
                                                                .flatten()
                                                                .collect();
                                                            variables_map.insert(
                                                                var_name,
                                                                CellArgument::Vector(vector),
                                                            );
                                                        } else {
                                                            variables_map.insert(
                                                                var_name,
                                                                CellArgument::Matrix(matrix),
                                                            );
                                                        }
                                                    }
                                                }
                                                let evaluated_value =
                                                    new_cell_expr.evaluate(&variables_map);

                                                let cell_id_string = format!(
                                                    "{}{}",
                                                    column_number_to_name(cell_identifier.col),
                                                    cell_identifier.row + 1
                                                );
                                                spreadsheet_guard.set_cell(
                                                    cell_id_string.clone(), // Clone for insertion
                                                    cell_expr.clone(), // Store the raw expression string
                                                    dependencies,
                                                    evaluated_value,
                                                    current_version,
                                                );

                                                // Notify dependents to re-evaluate
                                                // For Stage 4, only direct dependents (one-level deep)
                                                let mut dependents_to_notify: HashSet<String> =
                                                    HashSet::new();
                                                if let Some(entry) =
                                                    spreadsheet_guard.cells.get(&cell_id_string)
                                                {
                                                    dependents_to_notify
                                                        .extend(entry.dependents.iter().cloned());
                                                }
                                                drop(spreadsheet_guard); // Release lock before sending messages

                                                for dependent_id in dependents_to_notify {
                                                    sender_clone
                                                        .send((dependent_id, current_version))
                                                        .unwrap();
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let reply = Reply::Error(e.to_string());
                                        match send.write_message(reply) {
                                            WriteMessageResult::Ok => { /* Message successfully sent, continue. */
                                            }
                                            WriteMessageResult::ConnectionClosed => {
                                                break;
                                            }
                                            WriteMessageResult::Err(e) => {
                                                return Err(e.into());
                                            }
                                        }
                                    }
                                }
                            }
                            ReadMessageResult::ConnectionClosed => {
                                break;
                            }
                            ReadMessageResult::Err(e) => {
                                return Err(e.into());
                            }
                        }
                    }
                    Ok(())
                });
                join_handles.push(handle);
            }
            Connection::NoMoreConnections => {
                break;
            }
        }
    }

    for handle in join_handles {
        handle.join().unwrap().unwrap();
    }

    Ok(())
}
