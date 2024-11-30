use crate::rendering::canvas::Canvas;

pub fn print_positions(
    canvas: &mut dyn Canvas,
    (offset_x, offset_y): (i32, i32),
    positions: Vec<(i32, i32)>,
) {
    for (x, y) in positions {
        canvas.set((x + offset_x) as usize, (y + offset_y) as usize);
    }
}
