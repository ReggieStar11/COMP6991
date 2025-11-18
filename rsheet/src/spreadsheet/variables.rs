use rsheet_lib::cell_expr::{CellArgument, CellExpr};
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use std::collections::HashMap;

use super::spreadsheet::Spreadsheet;
use crate::parsing::parse_cell_range;

/// Collects variable values for expression evaluation.
/// Handles scalar (A1), vector (A1_A3), and matrix (A1_B3) variables.
pub fn collect_variables(
    expr: &CellExpr,
    spreadsheet: &Spreadsheet,
) -> HashMap<String, CellArgument> {
    let mut variables = HashMap::new();

    for var_name in expr.find_variable_names() {
        let (start_id, end_id) = match parse_cell_range(&var_name) {
            Ok(ids) => ids,
            Err(e) => {
                variables.insert(var_name, CellArgument::Value(CellValue::Error(e)));
                continue;
            }
        };

        if start_id == end_id {
            // Scalar variable
            let cell_id = format!(
                "{}{}",
                column_number_to_name(start_id.col),
                start_id.row + 1
            );
            let value = spreadsheet.get_cell(&cell_id);
            variables.insert(var_name, CellArgument::Value(value));
        } else {
            // Vector or matrix variable
            let mut matrix = Vec::new();
            for row in start_id.row..=end_id.row {
                let mut row_vec = Vec::new();
                for col in start_id.col..=end_id.col {
                    let cell_id = format!("{}{}", column_number_to_name(col), row + 1);
                    row_vec.push(spreadsheet.get_cell(&cell_id));
                }
                matrix.push(row_vec);
            }

            if start_id.col == end_id.col || start_id.row == end_id.row {
                // Vector: single row or single column
                let vector = matrix.into_iter().flatten().collect();
                variables.insert(var_name, CellArgument::Vector(vector));
            } else {
                // Matrix: multiple rows and columns
                variables.insert(var_name, CellArgument::Matrix(matrix));
            }
        }
    }

    variables
}
