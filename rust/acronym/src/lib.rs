pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut prev = ' ';
    let mut word_start = true;

    for curr in phrase.chars() {
        match curr {
            ' ' | '-' | '_' => word_start = true,
            char if char.is_alphabetic() => {
                if word_start || (prev.is_lowercase() && curr.is_uppercase()) {
                    acronym.push(char.to_ascii_uppercase());
                    word_start = false;
                }
            }
            _ => (),
        }

        prev = curr;
    }

    return acronym;
}
