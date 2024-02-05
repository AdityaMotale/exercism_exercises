/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut code: Vec<char> = isbn
        .chars()
        .filter(|i| i.is_numeric() || *i == 'X')
        .collect();

    if code.len() != 10 || code.iter().position(|&x| x == 'X').map_or(false, |x| x < 9) {
        return false;
    }

    let mut sum = 0;
    let mut count = 10;

    for i in code {
        if let Some(digit) = i.to_digit(10) {
            sum += digit * count;
        } else if i == 'X' && count < 10 {
            sum += 10 * count;
        }

        if count > 0 {
            count -= 1;
        }
    }

    sum % 11 == 0
}
