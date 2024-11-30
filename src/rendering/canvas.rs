pub trait Canvas {
    fn size(&self) -> (usize, usize);

    fn char_size(&self) -> (u16, u16);
    fn set_char_size(&mut self, size: (u16, u16));

    fn set(&mut self, x: usize, y: usize);
    fn clear(&mut self);

    fn render(&self, x: u16, y: u16, output: &mut dyn std::io::Write);
}
