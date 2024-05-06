pub fn square(n: u32) -> u64 {
    return match n {
        1..=64 => 1 << (n - 1),
        _ => panic!("Square must be between 1 and 64"),
    };
}

pub fn total() -> u64 {
    return u64::MAX;
}

