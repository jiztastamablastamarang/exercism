pub fn series(digits: &str, len: usize) -> Vec<String> {
    return digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|w| w.iter().collect::<String>())
        .collect();
}
