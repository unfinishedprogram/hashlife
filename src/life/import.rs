// Utilities convert from standard life representations to hash-life

pub fn rle_to_cell_positions(rle: String, offset_x: i64, offset_y: i64) -> Vec<(i64, i64)> {
    // Remove comments and size header
    let s: String = rle
        .lines()
        .filter(|l| !l.starts_with('#'))
        .skip(1)
        .collect();

    let mut cells = vec![];

    let body = s;

    let mut x = offset_x;
    let mut y = offset_y;

    let mut run_length = 1;
    let mut run_length_chars = 0;

    for c in body.chars() {
        match c {
            'o' => {
                for x in x..x + run_length {
                    cells.push((x, y));
                }
                x += run_length;
                run_length_chars = 0;
                run_length = 1;
            }
            'b' => {
                x += run_length;
                run_length_chars = 0;
                run_length = 1;
            }
            '$' => {
                y += run_length;
                run_length_chars = 0;
                x = offset_x;
                run_length = 1;
            }
            d if d.is_numeric() => {
                let n = c.to_digit(10).unwrap();
                if run_length_chars == 0 {
                    run_length = 0;
                }
                run_length_chars += 1;

                run_length *= 10;
                run_length += n as i64;
            }
            _ => {}
        }
    }

    cells
}
