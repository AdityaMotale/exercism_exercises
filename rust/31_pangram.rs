/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let char_set: Vec<char> = sentence.to_lowercase().chars().collect();

    ('a'..='z').all(|c| char_set.contains(&c))
}
