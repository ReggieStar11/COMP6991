use rsheet_lib::cell_expr::CellExprEvalError;
use rsheet_lib::cell_value::CellValue;
use std::collections::{HashMap, HashSet};

use super::graph::{add_dependencies, remove_dependencies};

pub struct CellEntry {
    pub expr_string: String,
    pub dependencies: HashSet<String>,
    pub dependents: HashSet<String>,
    pub value: CellValue,
    pub version: u64,
}

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
        let cell_value = match value {
            Ok(cell_value) => cell_value,
            Err(e) => match e {
                CellExprEvalError::VariableDependsOnError => {
                    CellValue::Error("DEPENDENCY_ERROR_MARKER".to_string())
                }
            },
        };

        // Check version to stop older updates from overwriting newer ones
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
                existing_dependents = old_entry.dependents.clone();
            }

            // Update dependency graph
            remove_dependencies(&mut self.cells, &old_dependencies, &cell_identifier);

            self.cells.insert(
                cell_identifier.clone(),
                CellEntry {
                    expr_string,
                    dependencies: new_dependencies.clone(),
                    dependents: existing_dependents,
                    value: cell_value,
                    version,
                },
            );

            add_dependencies(&mut self.cells, &new_dependencies, &cell_identifier);
        }
    }

    pub fn get_next_version(&mut self) -> u64 {
        let version = self.next_version;
        self.next_version += 1;
        version
    }

    pub fn get_cell_entry(&self, cell_identifier: &str) -> Option<&CellEntry> {
        self.cells.get(cell_identifier)
    }
}
