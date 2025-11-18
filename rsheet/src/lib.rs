use rsheet_lib::cell_expr::{CellArgument, CellExpr, CellExprEvalError};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use rsheet_lib::command::{CellIdentifier, Command};
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;
use std::collections::HashMap;

use std::error::Error;

use log::info;

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let mut spreadsheet = Spreadsheet::new();
    // This initiates a single client connection, and reads and writes messages
    // indefinitely.
    let (mut recv, mut send) = match manager.accept_new_connection() {
        Connection::NewConnection { reader, writer } => (reader, writer),
        Connection::NoMoreConnections => {
            // There are no more new connections to accept.
            return Ok(());
        }
    };
    loop {
        info!("Just got message");
        match recv.read_message() {
            ReadMessageResult::Message(msg) => {
                let command_result = msg.parse::<Command>();

                match command_result {
                    Ok(command) => {
                        match command {
                            Command::Get { cell_identifier } => {
                                let cell_id_string = format!(
                                    "{}{}",
                                    column_number_to_name(cell_identifier.col),
                                    cell_identifier.row + 1
                                );
                                let value = spreadsheet.get_cell(&cell_id_string);
                                let reply = Reply::Value(cell_id_string, value);
                                match send.write_message(reply) {
                                    WriteMessageResult::Ok => { /* Message successfully sent, continue. */
                                    }
                                    WriteMessageResult::ConnectionClosed => {
                                        break;
                                    }
                                    WriteMessageResult::Err(e) => {
                                        return Err(Box::new(e));
                                    }
                                }
                            }
                            Command::Set {
                                cell_identifier,
                                cell_expr,
                            } => {
                                // For Stage 1, we don't have dependencies, so no variables are passed.
                                let new_cell_expr = CellExpr::new(&cell_expr);

                                let mut variables_map: HashMap<String, CellArgument> =
                                    HashMap::new();

                                for var_name in new_cell_expr.find_variable_names() {
                                    let (start_id, end_id) = match parse_cell_range(&var_name) {
                                        Ok(ids) => ids,
                                        Err(e) => {
                                            // If a variable name itself is invalid, store an error for it
                                            variables_map.insert(
                                                var_name,
                                                CellArgument::Value(CellValue::Error(e)),
                                            );
                                            continue;
                                        }
                                    };

                                    if start_id == end_id {
                                        // Scalar variable
                                        let cell_id_string = format!(
                                            "{}{}",
                                            column_number_to_name(start_id.col),
                                            start_id.row + 1
                                        );
                                        let value = spreadsheet.get_cell(&cell_id_string);
                                        variables_map.insert(var_name, CellArgument::Value(value));
                                    } else {
                                        // Vector or Matrix variable
                                        let mut matrix = Vec::new();
                                        for row in start_id.row..=end_id.row {
                                            let mut row_vec = Vec::new();
                                            for col in start_id.col..=end_id.col {
                                                let cell_id_string = format!(
                                                    "{}{}",
                                                    column_number_to_name(col),
                                                    row + 1
                                                );
                                                let value = spreadsheet.get_cell(&cell_id_string);
                                                row_vec.push(value);
                                            }
                                            matrix.push(row_vec);
                                        }

                                        if start_id.col == end_id.col || start_id.row == end_id.row
                                        {
                                            // It's a vector (single column or single row)
                                            // Flatten the matrix into a single vector
                                            let vector = matrix.into_iter().flatten().collect();
                                            variables_map
                                                .insert(var_name, CellArgument::Vector(vector));
                                        } else {
                                            // It's a matrix
                                            variables_map
                                                .insert(var_name, CellArgument::Matrix(matrix));
                                        }
                                    }
                                }

                                let evaluated_value = new_cell_expr.evaluate(&variables_map);

                                let cell_id_string = format!(
                                    "{}{}",
                                    column_number_to_name(cell_identifier.col),
                                    cell_identifier.row + 1
                                );
                                spreadsheet.set_cell(cell_id_string, evaluated_value);
                                // Successful set command has no output, so we don't send a reply.
                            }
                        }
                    }
                    Err(e) => {
                        let reply = Reply::Error(e.to_string());
                        match send.write_message(reply) {
                            WriteMessageResult::Ok => { /* Message successfully sent, continue. */ }
                            WriteMessageResult::ConnectionClosed => {
                                break;
                            }
                            WriteMessageResult::Err(e) => {
                                return Err(Box::new(e));
                            }
                        }
                    }
                }
            }
            ReadMessageResult::ConnectionClosed => {
                // The connection was closed. This is not an error, but
                // should terminate this connection.
                break;
            }
            ReadMessageResult::Err(e) => {
                // An unexpected error was encountered.
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}

// A simple struct to hold our spreadsheet data.
struct Spreadsheet {
    cells: std::collections::HashMap<String, CellValue>,
}

impl Spreadsheet {
    fn new() -> Self {
        Spreadsheet {
            cells: std::collections::HashMap::new(),
        }
    }

    fn get_cell(&self, cell_identifier: &str) -> CellValue {
        self.cells
            .get(cell_identifier)
            .cloned()
            .unwrap_or(CellValue::None)
    }

    fn set_cell(
        &mut self,
        cell_identifier: String,
        value: Result<CellValue, rsheet_lib::cell_expr::CellExprEvalError>,
    ) {
        match value {
            Ok(cell_value) => {
                self.cells.insert(cell_identifier, cell_value);
            }
            Err(e) => {
                // In Stage 1, we just store the error as a CellValue::Error for now.
                // In later stages, we'll need to distinguish this from CellValue::Error produced by Rhai.
                self.cells
                    .insert(cell_identifier, CellValue::Error(format!("{:?}", e)));
            }
        }
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
    let parts: Vec<&str> = s.split_inclusive(char::is_alphabetic).collect();
    if parts.len() != 2 {
        return Err(format!("Invalid cell identifier format: {}", s));
    }

    let col_str = parts[0];
    let row_str = parts[1];

    let col = column_name_to_number(col_str)?;
    let row = row_str.parse::<u32>().map_err(|e| e.to_string())?;

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
