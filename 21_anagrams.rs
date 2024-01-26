use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sort_string(&lowercase_word);

    possible_anagrams
        .iter()
        .filter(|&&s| {
            let lowercase_s = s.to_lowercase();
            lowercase_word != lowercase_s && sorted_word == sort_string(&lowercase_s)
        })
        .copied()
        .collect()
}

pub fn sort_string(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars
}
