pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            '}' | ']' | ')' => {
                if stack.is_empty() || !is_matching_pair(stack.pop().unwrap(), c) {
                    return false;
                }
            }
            _ => ()
        }
    }

    stack.is_empty()
}

fn is_matching_pair(open: char, close: char) -> bool {
    match (open, close) {
        ('[', ']') | ('{', '}') | ('(', ')') => true,
        _ => false,
    }
}
