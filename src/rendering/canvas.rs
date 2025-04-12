use std::io::Write;

pub trait Canvas {
    fn size(&self) -> (usize, usize);

    fn char_size(&self) -> (u16, u16);
    fn set_char_size(&mut self, size: (u16, u16));

    fn set(&mut self, x: usize, y: usize);
    fn clear(&mut self);

    fn render(&self, output: &mut impl Write);
}
