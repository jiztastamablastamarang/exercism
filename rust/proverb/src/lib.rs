pub fn build_proverb(list: &[&str]) -> String {
    return if list.is_empty() { String::new() } else {
        list.windows(2)
            .map(|pair| format!("For want of a {prev} the {next} was lost.", prev = pair[0], next = pair[1]))
            .chain(std::iter::once(format!("And all for the want of a {first}.", first=list[0])))
            .collect::<Vec<String>>()
            .join("\n")
    };
}