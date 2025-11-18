use log::info;
use rsheet_lib::cell_expr::CellExprEvalError;
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::cells::column_number_to_name;
use std::collections::{HashMap, HashSet};

use crate::parsing::parse_cell_range;

// Represents the state of a single cell in the spreadsheet.
pub struct CellEntry {
    pub expr_string: String,
    pub dependencies: HashSet<String>, // Names of cells this cell directly depends on
    pub dependents: HashSet<String>,   // Names of cells that depend on this cell
    pub value: CellValue,
    pub version: u64, // For handling temporal ordering of updates
}

// A simple struct to hold our spreadsheet data.
pub struct Spreadsheet {
    cells: HashMap<String, CellEntry>,
    next_version: u64,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Spreadsheet {
            cells: HashMap::new(),
            next_version: 0,
        }
    }

    pub fn get_cell(&self, cell_identifier: &str) -> CellValue {
        self.cells
            .get(cell_identifier)
            .map(|entry| entry.value.clone())
            .unwrap_or(CellValue::None)
    }

    pub fn set_cell(
        &mut self,
        cell_identifier: String,
        expr_string: String,
        new_dependencies: HashSet<String>,
        value: Result<CellValue, CellExprEvalError>,
        version: u64,
    ) {
        info!(
            "set_cell: cell={}, expr='{}', new_deps={:?}, version={}",
            cell_identifier, expr_string, new_dependencies, version
        );
        let cell_value = match value {
            Ok(cell_value) => cell_value,
            Err(e) => match e {
                CellExprEvalError::VariableDependsOnError => {
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
            let mut existing_dependents = HashSet::new();
            if let Some(old_entry) = self.cells.get(&cell_identifier) {
                old_dependencies = old_entry.dependencies.clone();
                existing_dependents = old_entry.dependents.clone(); // Preserve existing dependents
            }

            // Remove this cell from the dependents of its old dependencies
            // For ranges (e.g., "A1_A3"), remove from dependents of all cells in the range
            for dep in &old_dependencies {
                match parse_cell_range(dep) {
                    Ok((start_id, end_id)) => {
                        // Expand the range into individual cells
                        for row in start_id.row..=end_id.row {
                            for col in start_id.col..=end_id.col {
                                let dep_cell_string =
                                    format!("{}{}", column_number_to_name(col), row + 1);
                                if let Some(dep_entry) = self.cells.get_mut(&dep_cell_string) {
                                    info!(
                                        "set_cell: Removing {} from dependents of {}",
                                        cell_identifier, dep_cell_string
                                    );
                                    dep_entry.dependents.remove(&cell_identifier);
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // If parsing fails, treat as a single cell (shouldn't happen, but handle gracefully)
                        if let Some(dep_entry) = self.cells.get_mut(dep) {
                            info!(
                                "set_cell: Removing {} from dependents of {}",
                                cell_identifier, dep
                            );
                            dep_entry.dependents.remove(&cell_identifier);
                        }
                    }
                }
            }

            // Insert the new cell entry, preserving existing dependents
            self.cells.insert(
                cell_identifier.clone(),
                CellEntry {
                    expr_string,
                    dependencies: new_dependencies.clone(),
                    dependents: existing_dependents, // Preserve existing dependents
                    value: cell_value,
                    version,
                },
            );

            // Add this cell to the dependents of its new dependencies
            // For ranges (e.g., "A1_A3"), add to dependents of all cells in the range
            for dep in &new_dependencies {
                match parse_cell_range(dep) {
                    Ok((start_id, end_id)) => {
                        // Expand the range into individual cells
                        for row in start_id.row..=end_id.row {
                            for col in start_id.col..=end_id.col {
                                let dep_cell_string =
                                    format!("{}{}", column_number_to_name(col), row + 1);
                                if let Some(dep_entry) = self.cells.get_mut(&dep_cell_string) {
                                    info!(
                                        "set_cell: Adding {} to dependents of {}",
                                        cell_identifier, dep_cell_string
                                    );
                                    dep_entry.dependents.insert(cell_identifier.clone());
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // If parsing fails, treat as a single cell (shouldn't happen, but handle gracefully)
                        if let Some(dep_entry) = self.cells.get_mut(dep) {
                            info!(
                                "set_cell: Adding {} to dependents of {}",
                                cell_identifier, dep
                            );
                            dep_entry.dependents.insert(cell_identifier.clone());
                        }
                    }
                }
            }
        }
    }

    // Helper to get the next version number atomically
    pub fn get_next_version(&mut self) -> u64 {
        let current_version = self.next_version;
        self.next_version += 1;
        current_version
    }

    pub fn get_cell_entry(&self, cell_identifier: &str) -> Option<&CellEntry> {
        self.cells.get(cell_identifier)
    }
}
