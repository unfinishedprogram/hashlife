use std::io::{StdoutLock, Write};

use crossterm::{cursor::MoveTo, QueueableCommand};

use super::canvas::Canvas;

pub struct DetailedCanvas {
    char_size: (u16, u16),
    buffer: Vec<u8>,
}

impl DetailedCanvas {
    const EMPTY_BRAIL: char = '⠀';
    const BRAIL_WIDTH: u16 = 2;
    const BRAIL_HEIGHT: u16 = 4;

    pub fn new((width, height): (u16, u16)) -> Self {
        DetailedCanvas {
            buffer: Self::create_buffer((width, height)),
            char_size: (width, height),
        }
    }

    fn create_buffer(char_size: (u16, u16)) -> Vec<u8> {
        vec![0; (char_size.0 as usize) * (char_size.1 as usize)]
    }

    fn char_position(pixel_position: (usize, usize)) -> (u16, u16) {
        let (x, y) = pixel_position;
        (x as u16 / Self::BRAIL_WIDTH, y as u16 / Self::BRAIL_HEIGHT)
    }

    fn char_index(&self, char_position: (u16, u16)) -> usize {
        (char_position.1 * self.char_size.0 + char_position.0) as usize
    }

    fn brail_bit(pixel_position: (usize, usize)) -> u8 {
        let (x, y) = pixel_position;
        let (x, y) = (x as u16 % Self::BRAIL_WIDTH, y as u16 % Self::BRAIL_HEIGHT);

        let bit_offset = match (x, y) {
            (0, 0) => 0,
            (0, 1) => 1,
            (0, 2) => 2,
            (1, 0) => 3,
            (1, 1) => 4,
            (1, 2) => 5,
            (0, 3) => 6,
            (1, 3) => 7,
            _ => unreachable!(),
        };

        1 << bit_offset
    }
}

impl Canvas for DetailedCanvas {
    fn size(&self) -> (usize, usize) {
        let (width, height) = self.char_size;
        (
            (width * Self::BRAIL_WIDTH) as usize,
            (height * Self::BRAIL_HEIGHT) as usize,
        )
    }

    fn char_size(&self) -> (u16, u16) {
        self.char_size
    }

    fn set_char_size(&mut self, size: (u16, u16)) {
        self.char_size = size;
        self.buffer = Self::create_buffer(size);
    }

    fn set(&mut self, x: usize, y: usize) {
        let (max_x, max_y) = self.size();
        if x >= max_x || y >= max_y {
            return;
        }

        let char_position = Self::char_position((x, y));
        let index = self.char_index(char_position);
        self.buffer[index] |= Self::brail_bit((x, y));
    }

    fn clear(&mut self) {
        for c in &mut self.buffer {
            *c = 0;
        }
    }

    fn render(&self, offset_x: u16, offset_y: u16, output: &mut StdoutLock) {
        let (width, height) = self.char_size();

        for y in 0..height {
            let mut buffer = String::new();
            output.queue(MoveTo(offset_x, offset_y + y)).unwrap();

            for x in 0..width {
                let index = self.char_index((x, y));
                let c = self.buffer[index];
                let c = std::char::from_u32(Self::EMPTY_BRAIL as u32 + c as u32).unwrap();
                buffer.push(c);
            }

            output.write_all(buffer.as_bytes()).unwrap()
        }
    }
}
