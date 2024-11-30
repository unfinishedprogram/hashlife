use crate::rendering::canvas::Canvas;

pub fn print_positions(
    canvas: &mut dyn Canvas,
    (offset_x, offset_y): (i32, i32),
    positions: Vec<(i32, i32)>,
) {
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
    min_y -= 1;

    for (x, y) in positions {
        canvas.set(
            (x - min_x + offset_x) as usize,
            (y - min_y + offset_y) as usize,
        );
    }
}
