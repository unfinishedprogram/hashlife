use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    terminal::{BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate},
    QueueableCommand,
};
use hashlife::life::{print::print_positions, Life};

fn main() {
    println!("Starting");

    let mut life = Life::from_rle(include_str!("../../patterns/clock.rle"));

    let stdout = stdout();
    loop {
        let start = std::time::Instant::now();

        life.step();
        let deep = 10;
        let depth = life.root.layer().max(deep) - deep;

        let mut str = print_positions(life.cell_positions(depth as u8));
        str.push_str(&format!("Alive: {}\n", life.root.alive()));
        str.push_str(&format!("Step time: {:?}\n", start.elapsed()));

        let mut stdout = stdout.lock();

        stdout.queue(BeginSynchronizedUpdate).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(MoveTo(0, 0)).unwrap();
        stdout.write_all(str.as_bytes()).unwrap();
        stdout.queue(EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }
}
