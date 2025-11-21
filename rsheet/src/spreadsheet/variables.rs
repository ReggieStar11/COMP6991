use rsheet_lib::cell_expr::{CellArgument, CellExpr};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use std::collections::HashMap;

use super::spreadsheet::Spreadsheet;
use crate::parsing::parse_cell_range;

pub fn collect_variables(
    expr: &CellExpr,
    spreadsheet: &Spreadsheet,
) -> HashMap<String, CellArgument> {
    let mut variables = HashMap::new();

    for var_name in expr.find_variable_names() {
        let (start_id, end_id) = match parse_cell_range(&var_name) {
            Ok(ids) => ids,
            Err(e) => {
                // Invalid cell range, mark as error
                variables.insert(var_name, CellArgument::Value(CellValue::Error(e)));
                continue;
            }
        };

        if start_id == end_id {
            // Single cell (scalar)
            let cell_id = format!(
                "{}{}",
                column_number_to_name(start_id.col),
                start_id.row + 1
            );
            let value = spreadsheet.get_cell(&cell_id);
            variables.insert(var_name, CellArgument::Value(value));
        } else {
            // Build matrix from cell range
            let mut matrix = Vec::new();
            for row in start_id.row..=end_id.row {
                let mut row_vec = Vec::new();
                for col in start_id.col..=end_id.col {
                    let cell_id = format!("{}{}", column_number_to_name(col), row + 1);
                    row_vec.push(spreadsheet.get_cell(&cell_id));
                }
                matrix.push(row_vec);
            }

            // Check if it's a vector (single row/column) or matrix
            if start_id.col == end_id.col || start_id.row == end_id.row {
                let vector = matrix.into_iter().flatten().collect();
                variables.insert(var_name, CellArgument::Vector(vector));
            } else {
                variables.insert(var_name, CellArgument::Matrix(matrix));
            }
        }
    }

    variables
}
