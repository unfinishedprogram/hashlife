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

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let mut grid = vec![vec!['.'; width]; height];

    for (x, y) in positions {
        let x = (x - min_x) as usize;
        let y = (y - min_y) as usize;

        grid[y][x] = '#';
    }

    for row in grid {
        println!("{}", row.into_iter().collect::<String>());
    }
}
