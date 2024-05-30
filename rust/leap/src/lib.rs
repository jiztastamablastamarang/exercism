pub fn is_leap_year(year: u64) -> bool {
    let year = match year {
        y if y % 100 == 0 => y / 100,
        y => y
    };

    return year % 4 == 0;
}
