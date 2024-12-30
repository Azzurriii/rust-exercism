pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = minefield[0].len();

    let mut result = vec![vec![' '; cols]; rows];

    for (i, row) in minefield.iter().enumerate() {
        for (j, cell) in row.chars().enumerate() {
            if cell == '*' {
                result[i][j] = '*';
            } else {
                let mut count = 0;
                for x in (i as i32 - 1)..=(i as i32 + 1) {
                    for y in (j as i32 - 1)..=(j as i32 + 1) {
                        if x >= 0
                            && x < rows as i32
                            && y >= 0
                            && y < cols as i32
                            && !(x == i as i32 && y == j as i32)
                            && minefield[x as usize].chars().nth(y as usize) == Some('*')
                        {
                            count += 1;
                        }
                    }
                }
                if count > 0 {
                    result[i][j] = std::char::from_digit(count, 10).unwrap();
                }
            }
        }
    }

    result.iter().map(|row| row.iter().collect()).collect()
}