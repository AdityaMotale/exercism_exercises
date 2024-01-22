const MAX_SQUARE: u32 = 64;
const MIN_SQUARE: u32 = 1;

pub fn square(s: u32) -> u64 {
    if s < MIN_SQUARE || s > MAX_SQUARE {
        panic!("Square must be between {} and {}", MIN_SQUARE, MAX_SQUARE);
    }
    
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (MIN_SQUARE..=MAX_SQUARE).map(square).sum()
}
