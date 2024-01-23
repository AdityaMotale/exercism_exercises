pub fn private_key(mut p: u64) -> u64 {
    p / 2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    get_mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    get_mod_pow(b_pub, a, p)
}

fn get_mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    base = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }

    result
}
