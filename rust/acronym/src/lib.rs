pub fn abbreviate(phrase: &str) -> String {
    return phrase
        .split(&[' ', '-', '_'][..])
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|char| char.is_uppercase())
                    .filter(|char| char.is_uppercase()),
            )
        })
        .map(|char| char.to_ascii_uppercase())
        .collect::<String>();
}
