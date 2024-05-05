pub fn is_armstrong_number(num: u32) -> bool {
    return num
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|d| (d, num.to_string().len() as u32))
        .fold(0, |acc: u32, (d, n)| acc.checked_add(d.pow(n)).unwrap_or(0)) == num;
}


