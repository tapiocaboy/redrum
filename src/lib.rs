use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// Computes the SHA-256 hash of a string input
///
/// # Examples
///
/// ```
/// use redrum::hash_string;
///
/// let hash = hash_string("REDRUM");
/// assert_eq!(hash.len(), 64);
/// assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
/// ```
pub fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Computes SHA-256 hashes for each character in the input string
///
/// # Examples
///
/// ```
/// use redrum::hash_individual_chars;
///
/// let result = hash_individual_chars("ABC");
/// assert_eq!(result.len(), 3);
/// assert_eq!(result[0].0, 'A');
/// assert_eq!(result[1].0, 'B');
/// assert_eq!(result[2].0, 'C');
/// ```
pub fn hash_individual_chars(input: &str) -> Vec<(char, String)> {
    input.chars()
        .map(|c| (c, hash_string(&c.to_string())))
        .collect()
}

/// Calculates frequency of each character in a string
///
/// # Examples
///
/// ```
/// use redrum::calculate_frequency;
///
/// let freq = calculate_frequency("aabbcc");
/// assert_eq!(freq[&'a'], 2);
/// assert_eq!(freq[&'b'], 2);
/// assert_eq!(freq[&'c'], 2);
/// ```
pub fn calculate_frequency(hash: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in hash.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}

/// Gets the 0-based alphabet index of a character (A=0, B=1, ..., Z=25)
///
/// # Examples
///
/// ```
/// use redrum::get_alphabet_index;
///
/// assert_eq!(get_alphabet_index('A'), 0);
/// assert_eq!(get_alphabet_index('Z'), 25);
/// assert_eq!(get_alphabet_index('a'), 0); // Should handle lowercase
/// assert_eq!(get_alphabet_index('z'), 25); // Should handle lowercase
/// ```
pub fn get_alphabet_index(c: char) -> u8 {
    (c.to_ascii_uppercase() as u8) - b'A'
}

/// Analyzes combined hashes of a word and its individual characters
///
/// # Examples
///
/// ```
/// use redrum::analyze_combined_hashes;
///
/// let (word_hash, char_hashes, combined_hash) = analyze_combined_hashes("TEST");
/// assert_eq!(word_hash.len(), 64);
/// assert_eq!(char_hashes.len(), 4);
/// assert_eq!(combined_hash.len(), 64);
/// ```
pub fn analyze_combined_hashes(word: &str) -> (String, Vec<(char, String)>, String) {
    // Hash the entire word
    let word_hash = hash_string(word);
    
    // Hash individual characters
    let char_hashes = hash_individual_chars(word);
    
    // Concatenate individual character hashes
    let concatenated_hashes = char_hashes.iter()
        .map(|(_, hash)| hash.as_str())
        .collect::<Vec<&str>>()
        .join("");
    
    // Hash the concatenated hashes
    let combined_hash = hash_string(&concatenated_hashes);
    
    (word_hash, char_hashes, combined_hash)
}

/// Analyzes character shifts between different hash representations
///
/// # Examples
///
/// ```
/// use redrum::analyze_character_shifts;
///
/// let word = "TEST";
/// let word_hash = "a".repeat(64); // Mock hash
/// let combined_hash = "b".repeat(64); // Mock hash
/// let shifts = analyze_character_shifts(word, &word_hash, &combined_hash);
/// 
/// assert_eq!(shifts.len(), 4); // Should have same length as input word
/// assert_eq!(shifts[0].0, 'T'); // First character should be 'T'
/// assert_eq!(shifts[0].1, 19); // 'T' should have index 19
/// ```
pub fn analyze_character_shifts(word: &str, word_hash: &str, combined_hash: &str) -> Vec<(char, u8, char, char)> {
    let mut shifts = Vec::new();
    
    // Get character positions in word hash and combined hash
    for (i, c) in word.chars().enumerate() {
        let alphabet_index = get_alphabet_index(c);
        
        // Get corresponding characters from hashes (using modulo to handle different lengths)
        let word_hash_char = word_hash.chars().nth(i % word_hash.len()).unwrap_or('?');
        let combined_hash_char = combined_hash.chars().nth(i % combined_hash.len()).unwrap_or('?');
        
        shifts.push((c, alphabet_index, word_hash_char, combined_hash_char));
    }
    
    shifts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let result = hash_string("REDRUM");
        assert_eq!(result.len(), 64); // SHA-256 hash is always 64 characters in hex
        assert!(result.chars().all(|c| c.is_ascii_hexdigit())); // All characters should be hex digits
    }

    #[test]
    fn test_hash_individual_chars() {
        let result = hash_individual_chars("ABC");
        assert_eq!(result.len(), 3); // Should have same length as input
        assert_eq!(result[0].0, 'A'); // First character should be 'A'
        assert_eq!(result[1].0, 'B'); // Second character should be 'B'
        assert_eq!(result[2].0, 'C'); // Third character should be 'C'
    }

    #[test]
    fn test_calculate_frequency() {
        let freq = calculate_frequency("aabbcc");
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 2);
        assert_eq!(freq[&'c'], 2);
    }

    #[test]
    fn test_get_alphabet_index() {
        assert_eq!(get_alphabet_index('A'), 0);
        assert_eq!(get_alphabet_index('Z'), 25);
        assert_eq!(get_alphabet_index('a'), 0); // Should handle lowercase
        assert_eq!(get_alphabet_index('z'), 25); // Should handle lowercase
    }

    #[test]
    fn test_analyze_combined_hashes() {
        let (word_hash, char_hashes, combined_hash) = analyze_combined_hashes("TEST");
        assert_eq!(word_hash.len(), 64);
        assert_eq!(char_hashes.len(), 4);
        assert_eq!(combined_hash.len(), 64);
    }

    #[test]
    fn test_analyze_character_shifts() {
        let word = "TEST";
        let word_hash = "a".repeat(64); // Mock hash
        let combined_hash = "b".repeat(64); // Mock hash
        let shifts = analyze_character_shifts(word, &word_hash, &combined_hash);
        
        assert_eq!(shifts.len(), 4); // Should have same length as input word
        assert_eq!(shifts[0].0, 'T'); // First character should be 'T'
        assert_eq!(shifts[0].1, 19); // 'T' should have index 19
    }
} 