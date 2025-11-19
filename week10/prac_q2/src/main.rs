use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};

// NOTE: You *may not* change the names or types of the members of this struct.
//       You may only add lifetime-relevant syntax.
pub struct SearchResult<'a, 'b> {
    pub matches: Vec<&'a str>,
    pub contains: &'b str,
}

/// Returns a [`SearchResult`] struct, where the matches vec is
/// a vector of every sentence that contains `contains`.
///
/// A sentence is defined as a slice of an `&str` which is the first
/// character of the string, or the first non-space character after
/// a full-stop (`.`), all the way until the last non-space character
/// before a full-stop or the end of the string.
///
/// For example, In the string "Hello. I am Tom . Goodbye", the three
/// sentences are "Hello", "I am Tom" and "Goodbye"
fn find_sentences_containing<'a, 'b>(text: &'a str, contains: &'b str) -> SearchResult<'a, 'b> {
    let mut matches = Vec::new();
    let mut start = 0;

    // Find all ". " patterns (period followed by space) to split sentences
    // Also handle periods followed by newlines or other whitespace
    let mut search_from = 0;
    while let Some(period_pos) = text[search_from..].find('.') {
        let period_abs = search_from + period_pos;

        // Check if there's whitespace after the period (sentence boundary)
        let after_period = period_abs + 1;
        if after_period < text.len() {
            let next_char = text[after_period..].chars().next();
            if let Some(ch) = next_char {
                if ch.is_whitespace() {
                    // This is a sentence boundary: period followed by whitespace
                    // Extract sentence from start to period, trim whitespace
                    let sentence_slice = &text[start..period_abs];
                    let trimmed = sentence_slice.trim();

                    if !trimmed.is_empty() && trimmed.contains(contains) {
                        // Find the actual bounds in the original text
                        let sentence_start = start + sentence_slice.find(trimmed).unwrap_or(0);
                        let sentence_end = sentence_start + trimmed.len();
                        matches.push(&text[sentence_start..sentence_end]);
                    }

                    // Find the start of the next sentence (first non-whitespace after period)
                    let mut next_start = after_period;
                    while next_start < text.len() {
                        let ch = text[next_start..].chars().next().unwrap();
                        if !ch.is_whitespace() {
                            break;
                        }
                        next_start += ch.len_utf8();
                    }
                    start = next_start;
                    search_from = next_start;
                    continue;
                }
            }
        }

        // Not a sentence boundary (e.g., "13.8"), continue searching from after this period
        // but don't update start (sentence start remains the same)
        search_from = period_abs + 1;
    }

    // Handle the last sentence (if text doesn't end with a period)
    if start < text.len() {
        let trimmed = text[start..].trim();
        if !trimmed.is_empty() && trimmed.contains(contains) {
            let sentence_start = start + text[start..].find(trimmed).unwrap_or(0);
            let sentence_end = sentence_start + trimmed.len();
            matches.push(&text[sentence_start..sentence_end]);
        }
    }

    SearchResult { matches, contains }
}

/// Given a vec of [`SearchResult`]s, return a hashmap, which lists how many
/// time each sentence occured in the search results.
fn count_sentence_matches<'a>(searches: Vec<SearchResult<'a, '_>>) -> HashMap<&'a str, i32> {
    let mut map = HashMap::new();
    for search in searches {
        for sentence in search.matches {
            *map.entry(sentence).or_insert(0) += 1;
        }
    }
    map
}

/////////// DO NOT CHANGE BELOW HERE ///////////

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let text = fs::read_to_string(file_path)?;

    let mut sentence_matches = {
        let mut found = vec![];

        let stdin = io::stdin();
        let matches = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        for line in matches.iter() {
            let search_result = find_sentences_containing(&text, line);
            println!(
                "Found {} results for '{}'.",
                search_result.matches.len(),
                search_result.contains
            );
            found.push(search_result);
        }

        count_sentence_matches(found)
            .into_iter()
            .collect::<Vec<_>>()
    };
    sentence_matches.sort();

    for (key, value) in sentence_matches {
        println!("'{}' occured {} times.", key, value);
    }

    Ok(())
}
