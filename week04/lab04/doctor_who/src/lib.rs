//! # Doctor Who Caesar Cipher Library
//! 
//! This crate provides functionality for Caesar cipher operations

/// The default shift value when none is specified
const DEFAULT_SHIFT: i32 = 5;

/// ASCII value of uppercase 'A'
const UPPERCASE_A: i32 = 65;

/// ASCII value of lowercase 'A'
const LOWERCASE_A: i32 = 97;

/// The value of alphabet size
const ALPHABET_SIZE: i32 = 26;

/// Applies Caesar cipher to multiple lines of text.
/// 
/// Takes an optional shift value and vector of strings.
/// 
/// ## Examples
/// 
/// ```
/// let input = vec!["Hello".to_string()];
/// let result = doctor_who::caesar_shift(Some(3), input);
/// assert_eq!(result, vec!["Khoor".to_string()]);
/// ```
/// 
/// ## Returns
/// `Vec<String>`

pub fn caesar_shift(shift_by: Option<i32>, lines: Vec<String>) -> Vec<String> {
    let shift_number = shift_by.unwrap_or(DEFAULT_SHIFT);
    
    lines
        .iter()
        .map(|line| shift(shift_number, line.to_string()))
        .collect()
}

fn shift(shift_by: i32, line: String) -> String {
    let mut result: Vec<char> = Vec::new();

    // turn shift_by into a positive number between 0 and 25
    let shift_by = shift_by % ALPHABET_SIZE + ALPHABET_SIZE;

    line.chars().for_each(|c| {
        let ascii = c as i32;

        if ('A'..='Z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - UPPERCASE_A) + shift_by, ALPHABET_SIZE) + UPPERCASE_A,
            ));
        } else if ('a'..='z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - LOWERCASE_A) + shift_by, ALPHABET_SIZE) + LOWERCASE_A,
            ));
        } else {
            result.push(c)
        }
    });

    result.iter().collect()
}

fn abs_modulo(a: i32, b: i32) -> i32 {
    (a % b).abs()
}

fn to_ascii(i: i32) -> char {
    char::from_u32(i as u32).unwrap()
}
