pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for bracket in string.chars() {
        match bracket {
            '(' | '[' | '{' => stack.push(bracket),
            ')' if stack.pop() != Some('(') => return false,
            ']' if stack.pop() != Some('[') => return false,
            '}' if stack.pop() != Some('{') => return false,
            _ => (),
        }
    }

    return stack.is_empty();
}