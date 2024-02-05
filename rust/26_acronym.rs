pub fn abbreviate(phrase: &str) -> String {
    let mut abbr = String::new();
    phrase.chars().fold((' ', &mut abbr), |(last, s), c| {
        if last == '-' && c != ' ' {
            s.push(c.to_ascii_uppercase());
        }
        if last == ' ' && c.is_ascii_alphabetic()
            || last != ' ' && !last.is_ascii_uppercase() && c.is_ascii_uppercase()
        {
            s.push(c.to_ascii_uppercase());
        }
        (c, s)
    });
    abbr
}
