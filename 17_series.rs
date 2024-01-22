pub fn series(digits: &str, len: usize) -> Vec<String> {
    if (len == 0) || (digits.len() < len) {
        return vec![String::from(""); 0];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|s| s.iter().collect())
        .collect()
}
