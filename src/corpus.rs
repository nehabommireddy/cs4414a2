//! Load words from a dictionary file. Words are normalized on load (lowercase,
//! ASCII letters only). Already implemented, do not edit.

use std::fs;
use std::io;
use std::path::Path;

/// Read a dictionary file (one word per line). Words are normalized on load.
pub fn load_words(path: &Path) -> io::Result<Vec<String>> {
    let text = fs::read_to_string(path)?;
    let mut words = Vec::new();
    for line in text.lines() {
        let word = normalize(line);
        if !word.is_empty() {
            words.push(word);
        }
    }
    Ok(words)
}

/// Lowercase and keep ASCII letters only.
pub fn normalize(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
