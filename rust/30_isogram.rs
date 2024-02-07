use std::collections::HashSet;

pub fn check(mut candidate: &str) -> bool {
    let mut hash_set: HashSet<char> = HashSet::new();

    candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .all(|c| hash_set.insert(c))
}
