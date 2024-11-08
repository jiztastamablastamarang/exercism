
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let i = [
        "Alice", "Bob", "Charlie",
        "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana",
        "Joseph", "Kincaid", "Larry",
    ].iter()
        .position(|x| *x == student)
        .expect("invalid student");

    return diagram
        .lines()
        .flat_map(|x| x.chars().skip(2 * i).take(2))
        .map(|c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => panic!("invalid plant"),
        })
        .collect();
}
