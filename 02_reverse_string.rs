pub fn reverse(input: &str) -> String {
    let mut strToVec: Vec<char> = input.chars().collect();
    strToVec.reverse();
    strToVec.into_iter().collect()
}
