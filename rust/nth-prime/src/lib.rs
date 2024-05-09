pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    let mut primes = Vec::new();
    let mut i = 2;
    
    while n >= primes.len() {
        if is_prime(i) {
            primes.push(i);
        }
        i += 1;
    }

    return primes[n];
}

fn is_prime(n: u32) -> bool {
    return match n {
        0 | 1 => false,
        2 => true,
        _ if n % 2 == 0 => false,
        _ => (3..=((n as f64).sqrt() as u32))
            .step_by(2)
            .all(|x| n % x != 0),
    };
}

fn _is_prime_brute_force(n: u32) -> bool {
    return !(2..n).any(|x| n % x == 0);
}
