pub fn brackets_are_balanced(string: &str) -> bool {
    return eliminate_brackets(filter_brackets(string)).is_empty();
}

fn filter_brackets(string: &str) -> String {
    return string
        .chars()
        .filter(|&c| matches!(c, '(' | ')' | '{' | '}' | '[' | ']'))
        .collect();
}

fn eliminate_brackets(mut string: String) -> String {
    for pair in ["()", "{}", "[]"] {
        if let Some(pos) = string.find(pair) {
            string.replace_range(pos..pos + 2, "");
            return eliminate_brackets(string);
        }
    }

    return string;
}

