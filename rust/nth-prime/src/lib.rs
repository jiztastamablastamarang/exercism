pub fn nth(n: u32) -> u32 {
    return (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap();
}

fn is_prime(n: u32) -> bool {
    return match n {
        0 | 1 => false,
        2 => true,
        _ if n % 2 == 0 => false,
        _ => (3..=(n as f64).sqrt() as u32)
            .step_by(2)
            .all(|x| n % x != 0),
    };
}