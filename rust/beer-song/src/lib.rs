fn bottles_label(n: u32) -> String {
    return if n == 1 { "bottle" } else { "bottles" }.to_string();
}

fn take_bottle_pronoun(n: u32) -> String {
    return if n == 1 { "it" } else { "one" }.to_string();
}

fn next_bottle_count(n: u32) -> String {
    return if n == 1 { "no more".to_string() } else { (n - 1).to_string() };
}

fn zero_bottle_lyrics() -> String {
    return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
}

fn non_zero_bottle_lyrics(n: u32) -> String {
    return format!(
        "{current} {current_label} of beer on the wall, {current} {current_label} of beer.\nTake {pronoun} down and pass it around, {next} {next_label} of beer on the wall.\n",
        current = n,
        current_label = bottles_label(n),
        pronoun = take_bottle_pronoun(n),
        next = next_bottle_count(n),
        next_label = bottles_label(if n == 1 { 0 } else { n - 1 })
    );
}

pub fn verse(n: u32) -> String {
    return match n {
        0 => zero_bottle_lyrics(),
        _ => non_zero_bottle_lyrics(n),
    };
}

pub fn sing(start: u32, end: u32) -> String {
    return (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n");
}

