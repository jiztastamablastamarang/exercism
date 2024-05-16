pub fn raindrops(n: u32) -> String {
    let result = [
        (n % 3 == 0, "Pling"),
        (n % 5 == 0, "Plang"),
        (n % 7 == 0, "Plong"),
    ]
        .iter()
        .filter(|&&(condition, _)| condition)
        .map(|&(_, sound)| sound)
        .collect::<String>();

    return if result.is_empty() { return n.to_string(); } else { result };
}

