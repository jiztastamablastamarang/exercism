use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    return factors
        .iter()
        .filter(|&&n| n != 0)
        .flat_map(|&n| {
            (1..)
                .map(move |i| n * i)
                .take_while(|&n| n < limit)
        })
        .collect::<HashSet<_>>()
        .iter()
        .sum();
}