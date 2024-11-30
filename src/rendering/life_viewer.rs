use std::io::StdoutLock;

use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyEvent, KeyEventKind},
    queue,
    style::Print,
    terminal::disable_raw_mode,
};

use crate::life::{print::print_positions, Life};

use super::{canvas::Canvas, detailed_canvas::DetailedCanvas};
enum RunningState {
    Running,
    Paused,
}

pub struct LifeViewer {
    life: Life,
    term_size: (u16, u16),
    running_state: RunningState,
    canvas: DetailedCanvas,
    render_depth: u8,

    cell_offset_x: i32,
    cell_offset_y: i32,
    speed: i32,
}

impl LifeViewer {
    pub fn new(term_size: (u16, u16), life: Life) -> Self {
        let mut res = LifeViewer {
            life,
            term_size,
            running_state: RunningState::Paused,
            canvas: DetailedCanvas::new(term_size),
            render_depth: 5,
            cell_offset_x: 0,
            cell_offset_y: 0,
            speed: 4,
        };
        res.center_on_zero_zero();
        res
    }

    fn center_on_zero_zero(&mut self) {
        self.cell_offset_x = self.pixel_scale() * (self.canvas.size().0 as i32) / 2;
        self.cell_offset_y = self.pixel_scale() * (self.canvas.size().1 as i32) / 2;
    }

    pub fn resize(&mut self, term_size: (u16, u16)) {
        self.term_size = term_size;
        self.canvas.set_char_size(term_size);
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Release {
            return;
        }

        match key.code {
            KeyCode::Esc => {
                disable_raw_mode().unwrap();
                std::process::exit(0)
            }
            KeyCode::Char(' ') => {
                self.running_state = match self.running_state {
                    RunningState::Running => RunningState::Paused,
                    RunningState::Paused => RunningState::Running,
                }
            }
            KeyCode::Char('=') => {
                if self.render_depth == u8::MIN {
                    return;
                }
                self.cell_offset_x -= self.pixel_scale() * (self.canvas.size().0 as i32) / 4;
                self.cell_offset_y -= self.pixel_scale() * (self.canvas.size().1 as i32) / 4;
                self.render_depth = self.render_depth.saturating_sub(1);
            }
            KeyCode::Char('-') => {
                if self.render_depth == u8::MAX {
                    return;
                }
                self.render_depth = self.render_depth.saturating_add(1);
                self.cell_offset_x += self.pixel_scale() * (self.canvas.size().0 as i32) / 4;
                self.cell_offset_y += self.pixel_scale() * (self.canvas.size().1 as i32) / 4;
            }
            KeyCode::Char('w') => {
                self.cell_offset_y += self.pixel_scale() * self.speed;
            }
            KeyCode::Char('s') => {
                self.cell_offset_y -= self.pixel_scale() * self.speed;
            }
            KeyCode::Char('a') => {
                self.cell_offset_x += self.pixel_scale() * self.speed;
            }
            KeyCode::Char('d') => {
                self.cell_offset_x -= self.pixel_scale() * self.speed;
            }
            _ => {}
        }
    }

    pub fn step(&mut self, output: &mut StdoutLock) {
        let step_start = std::time::Instant::now();
        if matches!(self.running_state, RunningState::Running) {
            self.life.step();
        }
        let step_time = step_start.elapsed();

        let render_start = std::time::Instant::now();
        self.render(output);
        let render_time = render_start.elapsed();

        queue!(
            output,
            MoveTo(0, 0),
            Print(&format!("Step: {:?}\n", step_time)),
            MoveTo(0, 1),
            Print(&format!("Render: {:?}\n", render_time)),
            MoveTo(0, 2),
            Print(&format!("Alive: {}\n", self.life.root.alive())),
            MoveTo(0, 3),
            Print(&format!("Zoom: 1/{}\n", self.pixel_scale())),
        )
        .unwrap();
    }

    pub fn pixel_scale(&self) -> i32 {
        1 << self.render_depth
    }

    fn render(&mut self, output: &mut StdoutLock) {
        let pixel_scale = self.pixel_scale();
        let positions = self.life.cell_positions(self.render_depth);
        self.canvas.clear();
        print_positions(
            &mut self.canvas,
            (
                self.cell_offset_x / pixel_scale,
                self.cell_offset_y / pixel_scale,
            ),
            positions,
        );
        self.canvas.render(0, 0, output);
    }
}
