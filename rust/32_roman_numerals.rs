use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

const ROMAN_LITERALS: &[(u32, &str); 13] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let roman_num = convert_to_roman(self.0);

        write!(_f, "{}", roman_num)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}

fn convert_to_roman(mut n: u32) -> String {
    let mut num_str = String::new();

    for &(value, symbol) in ROMAN_LITERALS.iter() {
        while n >= value {
            num_str += symbol;
            n -= value;
        }
    }

    return num_str;
}
