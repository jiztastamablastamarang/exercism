use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    gen_solutions(input)
        .into_iter()
        .find(|solution| match parts(&words(input)) {
            Some((left, right)) => is_solved(&left, right, solution),
            None => false,
        })
}

fn gen_solutions(evaluation: &str) -> Vec<HashMap<char, u8>> {
    let mut results = Vec::new();
    let words = words(evaluation);
    let vocabulary = vocabulary(&words);
    let first_chars = first_chars(&words);
    let mut stack = vec![(HashMap::new(), 0)];

    while let Some((mut solution, i)) = stack.pop() {
        if i == vocabulary.len() {
            results.push(solution);
            continue;
        }

        let char = vocabulary[i];
        let from = if first_chars.contains(&char) { 1 } else { 0 };

        (from..=9).for_each(|digit| {
            if !solution.values().any(|&val| val == digit) {
                solution.insert(char, digit);
                stack.push((solution.clone(), i + 1));
            }
        })
    }

    results
}

fn first_chars(words: &[&str]) -> HashSet<char> {
    words
        .iter()
        .flat_map(|word| word.trim().chars().next())
        .collect::<HashSet<char>>()
}

fn vocabulary(words: &[&str]) -> Vec<char> {
    words
        .iter()
        .flat_map(|word| word.chars())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect()
}

fn words(evaluation: &str) -> Vec<&str> {
    evaluation
        .split(&['+', '='][..])
        .map(|word| word.trim())
        .filter(|&word| word != "=")
        .collect()
}

fn parts<'a>(words: &[&'a str]) -> Option<(Vec<&'a str>, &'a str)> {
    words
        .split_last()
        .map(|(&right, left)| (left.to_vec(), right))
}

fn is_solved(left: &[&str], right: &str, solution: &HashMap<char, u8>) -> bool {
    left.iter()
        .map(|word| word_to_num(word, solution))
        .sum::<u64>()
        .eq(&word_to_num(right, solution))
}

fn word_to_num(word: &str, solution: &HashMap<char, u8>) -> u64 {
    word.chars()
        .map(|char| solution.get(&char).unwrap_or(&0))
        .fold(0, |acc, &digit| acc * 10 + digit as u64)
}
