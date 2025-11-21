use rsheet_lib::cells::column_number_to_name;
use std::collections::HashSet;

use super::spreadsheet::CellEntry;
use crate::parsing::parse_cell_range;

fn expand_dependency(dep: &str) -> Vec<String> {
    match parse_cell_range(dep) {
        Ok((start_id, end_id)) => {
            let mut cells = Vec::new();
            for row in start_id.row..=end_id.row {
                for col in start_id.col..=end_id.col {
                    cells.push(format!("{}{}", column_number_to_name(col), row + 1));
                }
            }
            cells
        }
        Err(_) => vec![dep.to_string()],
    }
}

fn update_dependency<F>(
    cells: &mut std::collections::HashMap<String, CellEntry>,
    dep: &str,
    mut f: F,
) where
    F: FnMut(&mut CellEntry),
{
    for expanded_cell in expand_dependency(dep) {
        if let Some(entry) = cells.get_mut(&expanded_cell) {
            f(entry);
        }
    }
}

pub fn remove_dependencies(
    cells: &mut std::collections::HashMap<String, CellEntry>,
    old_dependencies: &HashSet<String>,
    cell_id: &str,
) {
    for dep in old_dependencies {
        update_dependency(cells, dep, |entry| {
            entry.dependents.remove(cell_id);
        });
    }
}

pub fn add_dependencies(
    cells: &mut std::collections::HashMap<String, CellEntry>,
    new_dependencies: &HashSet<String>,
    cell_id: &str,
) {
    for dep in new_dependencies {
        update_dependency(cells, dep, |entry| {
            entry.dependents.insert(cell_id.to_string());
        });
    }
}
