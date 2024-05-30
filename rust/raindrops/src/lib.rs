pub fn raindrops(n: u32) -> String {
    let result = [
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong"),
    ]
        .iter()
        .filter(|(d, _)| n % d == 0)
        .map(|&(_, sound)| sound)
        .collect::<String>();

    return if result.is_empty() { return n.to_string(); } else { result };
}
