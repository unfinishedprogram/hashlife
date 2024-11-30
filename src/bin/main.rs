use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{size, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate},
    QueueableCommand,
};
use hashlife::{
    life::{print::print_positions, Life},
    rendering::detailed_canvas::DetailedCanvas,
};

fn main() {
    println!("Starting");
    let mut life = Life::from_rle(include_str!("../../patterns/clock.rle"));
    let mut canvas = DetailedCanvas::new(size().unwrap());
    let stdout = stdout();

    loop {
        let start = std::time::Instant::now();

        life.step();
        let deep = 10;
        let depth = life.root.layer().max(deep) - deep;

        let mut stdout = stdout.lock();

        stdout.queue(BeginSynchronizedUpdate).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        print_positions(&mut canvas, &mut stdout, life.cell_positions(depth as u8));

        queue!(
            stdout,
            MoveTo(0, 0),
            Print(&format!("Alive: {}\n", life.root.alive())),
            Print(&format!("Step time: {:?}\n", start.elapsed())),
            EndSynchronizedUpdate,
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}
