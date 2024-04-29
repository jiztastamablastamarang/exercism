pub fn reverse(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<char>>();
    let length = chars.len();

    for i in 0..length / 2 {
        chars.swap(i, length - i - 1);
    }

    return chars.into_iter().collect();
}


