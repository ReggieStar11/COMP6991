use rsheet_lib::command::CellIdentifier;

pub fn column_name_to_number(column_name: &str) -> Result<u32, String> {
    let mut col_num = 0;
    for char_code in column_name.to_uppercase().chars().map(|c| c as u32) {
        if !(('A' as u32)..=('Z' as u32)).contains(&char_code) {
            return Err(format!("Invalid character in column name: {}", column_name));
        }
        col_num = col_num * 26 + (char_code - 'A' as u32 + 1);
    }
    Ok(col_num - 1)
}

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
    Ok(CellIdentifier { col, row: row - 1 })
}

pub fn parse_cell_range(s: &str) -> Result<(CellIdentifier, CellIdentifier), String> {
    let parts: Vec<&str> = s.split('_').collect();
    match parts.len() {
        1 => {
            let cell_id = parse_cell_identifier_string(s)?;
            Ok((cell_id, cell_id))
        }
        2 => {
            let start_id = parse_cell_identifier_string(parts[0])?;
            let end_id = parse_cell_identifier_string(parts[1])?;
            Ok((start_id, end_id))
        }
        _ => Err(format!("Invalid cell range format: {}", s)),
    }
}

pub fn format_cell_identifier(cell_id: &rsheet_lib::command::CellIdentifier) -> String {
    format!(
        "{}{}",
        rsheet_lib::cells::column_number_to_name(cell_id.col),
        cell_id.row + 1
    )
}
