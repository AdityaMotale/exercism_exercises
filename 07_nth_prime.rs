pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    let mut number = 2;

    while (primes.len() as u32) <= n {
        let mut is_prime = true;

        // Calculate the square root of the current number for optimization.
        // Only factors up to the square root need to be checked.
        let sqrt = (number as f64).sqrt() as u32;
        
        for i in 2..=sqrt {
            if number % i == 0 {
                // If number is divisible by any number other than 1 and itself,
                // it is not prime.
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(number);
        }

        // Increment the number by 1 if it's 2, otherwise by 2.
        // This skips even numbers greater than 2, as they are not prime.
        number += if number == 2 { 1 } else { 2 };
    }

    *primes.last().unwrap()
}
