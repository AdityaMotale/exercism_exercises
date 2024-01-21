use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let numbers: HashSet<u32> = factors
        .iter()
        .filter(|&&n| n != 0)
        .flat_map(|&n| (n..limit).step_by(n as usize))
        .collect();

    numbers.iter().sum()
}
