/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let s2_chars: Vec<char> = s2.chars().collect();

    let mut diff: usize = 0;

    for (char1, char2) in s1.chars().zip(s2_chars) {
        if char1 != char2 {
            diff += 1;
        }
    }

    Some(diff)
}
