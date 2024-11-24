pub fn print_positions(positions: Vec<(i32, i32)>) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (x, y) in &positions {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let mut grid = vec![vec![0; width / 2 + 1]; height / 4 + 1];

    for (x, y) in positions {
        let char_offset_x = (x - min_x) as usize % 2;
        let char_offset_y = (y - min_y) as usize % 4;

        let x = (x - min_x) as usize / 2;
        let y = (y - min_y) as usize / 4;

        let c = &mut grid[y][x];

        let bit_offset = match (char_offset_x, char_offset_y) {
            (0, 0) => 0,
            (0, 1) => 1,
            (0, 2) => 2,
            (1, 0) => 3,
            (1, 1) => 4,
            (1, 2) => 5,
            (0, 3) => 6,
            (1, 3) => 7,
            (_, _) => unreachable!(),
        };

        let mask = 1 << bit_offset;
        *c |= mask;
    }

    for row in grid {
        for c in row {
            print!("{}", std::char::from_u32(0x2800 + c as u32).unwrap());
        }
        println!();
    }

    // group into 2x8 blocks for conversion to brail unicode
}
