use log::info;
use rsheet_lib::cell_expr::{CellArgument, CellExpr};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::parsing::parse_cell_range;
use crate::spreadsheet::Spreadsheet;

pub fn run_worker_thread(
    spreadsheet: Arc<Mutex<Spreadsheet>>,
    receiver: std::sync::mpsc::Receiver<(String, u64)>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for (cell_id_to_recalculate, triggering_version) in receiver {
        info!(
            "Worker: Received recalculation request for {} with triggering_version={}",
            cell_id_to_recalculate, triggering_version
        );
        let mut spreadsheet_guard = spreadsheet.lock().unwrap();

        if let Some(entry) = spreadsheet_guard.get_cell_entry(&cell_id_to_recalculate) {
            info!(
                "Worker: Processing {}. Current cell version={}, triggering_version={}",
                cell_id_to_recalculate, entry.version, triggering_version
            );
            // Check if this recalculation is based on an older version
            if triggering_version < entry.version {
                info!(
                    "Worker: Skipping {} because triggering_version < entry.version ({} < {}).",
                    cell_id_to_recalculate, triggering_version, entry.version
                );
                continue; // Skip if a newer version has already updated this cell
            }

            let expr_string = entry.expr_string.clone();
            let dependencies = entry.dependencies.clone();

            // Re-evaluate using current values of dependencies
            let new_cell_expr = CellExpr::new(&expr_string);
            let mut temp_spreadsheet_values = HashMap::new();
            for var_name in new_cell_expr.find_variable_names() {
                let (start_id, end_id) = match parse_cell_range(&var_name) {
                    Ok(ids) => ids,
                    Err(e) => {
                        temp_spreadsheet_values
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

            // Generate a NEW version number for this specific recalculation event
            let new_version_for_this_recalculation = spreadsheet_guard.get_next_version();

            spreadsheet_guard.set_cell(
                cell_id_to_recalculate.clone(),
                expr_string,
                dependencies,
                evaluated_value,
                new_version_for_this_recalculation, // Use the new version for the cell update
            );
        }
    }
    Ok(())
}
