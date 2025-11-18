use rsheet_lib::command::CellIdentifier;

// Helper function to parse a column name (e.g., "A", "AA") into a 0-indexed number.
pub fn column_name_to_number(column_name: &str) -> Result<u32, String> {
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
pub fn parse_cell_identifier_string(s: &str) -> Result<CellIdentifier, String> {
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
pub fn parse_cell_range(s: &str) -> Result<(CellIdentifier, CellIdentifier), String> {
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
