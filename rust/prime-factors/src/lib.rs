pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut prime_candidates = 2..;

    while n > 1 {
        let x = prime_candidates.next().unwrap();
        while n % x == 0 {
            n /= x;
            result.push(x);
        }
    }

    return result;
}

