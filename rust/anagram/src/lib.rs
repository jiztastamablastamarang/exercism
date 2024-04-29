use std::collections::HashSet;
use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower = word.to_lowercase();
    let sorted = lower.chars().sorted().collect::<String>();

    return possible_anagrams
        .iter()
        .filter(|&&x| {
            let x_lower = x.to_lowercase();
            let x_sorted = x_lower.chars().sorted().collect::<String>();
            lower != x_lower && sorted == x_sorted
        })
        .cloned()
        .collect();
}
