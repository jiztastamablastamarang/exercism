pub fn is_valid(code: &str) -> bool {
    return code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .map(|c| c.to_digit(10))
        .try_fold((0, 0), |(sum, i), d| match d {
            Some(d) => Ok((sum + if i % 2 == 0 { d } else { d * 2 - d / 5 * 9 }, i + 1)),
            None => Err(()),
        })
        .and_then(|(sum, i)| if i > 1 && sum % 10 == 0 { Ok(()) } else { Err(()) })
        .is_ok();
}
