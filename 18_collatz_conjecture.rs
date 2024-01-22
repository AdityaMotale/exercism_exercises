pub fn collatz(mut n: u64) -> Option<u64> {
    let mut steps = 0;

    while n > 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            n.checked_mul(3)?.checked_add(1)?
        };
        steps += 1;
    }

    (n != 0).then(|| steps)
}
