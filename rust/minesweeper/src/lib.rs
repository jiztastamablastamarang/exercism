pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::with_capacity(minefield.len());

    for (i, row) in minefield.iter().enumerate() {
        let mut row = row.as_bytes().to_vec();
        for (j, cell) in row.iter_mut().enumerate() {
            if !is_mine(cell) {
                if let Some(n) = count_mines(minefield, i, j) { *cell = b'0' + n }
            }
        }

        result.push(String::from_utf8(row).unwrap());
    }

    return result;
}

fn is_mine(x: &u8) -> bool { return x == &b'*'; }

fn define_range(x: usize, max: usize) -> impl Iterator<Item=usize> {
    return x.saturating_sub(1)..=(x + 1).min(max - 1);
}

fn count_mines(minefield: &[&str], row: usize, column: usize) -> Option<u8> {
    let rows_count = minefield.len();
    let columns_count = minefield.get(0)?.len();

    let mut count = 0;
    for i in define_range(row, rows_count) {
        for j in define_range(column, columns_count) {
            if is_mine(&minefield[i].as_bytes()[j]) {
                count += 1
            }
        }
    }

    if count == 0 { return None; };

    return Some(count);
}

