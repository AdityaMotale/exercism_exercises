use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut hash_map: HashMap<String, u32> = HashMap::new();

    words
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|c| !c.is_empty())
        .for_each(|word| {
            let word_str = word.trim_matches('\'').to_string();
            if !word_str.is_empty() {
                *hash_map.entry(word_str).or_insert(0) += 1;
            }
        });

    hash_map
}
