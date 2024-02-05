pub fn factors(mut num: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();

    // Early return for edge cases
    if num <= 1 {
        return prime_factors;
    }

    let mut factor = 2;

    // Limiting checks to the square root of the current number
    while factor * factor <= num {
        if num % factor == 0 {
            prime_factors.push(factor);
            num /= factor;
        } else {
            // Increment factor; 2 is the only even prime
            factor += if factor == 2 { 1 } else { 2 };
        }
    }

    // If num is not 1, it is a prime number itself
    if num > 1 {
        prime_factors.push(num);
    }

    prime_factors
}
