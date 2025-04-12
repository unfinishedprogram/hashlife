use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event},
    queue,
    terminal::{
        enable_raw_mode, size, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate,
    },
    QueueableCommand,
};
use hashlife::{life::Life, rendering::life_viewer::LifeViewer};

fn main() {
    let life = Life::from_rle(include_str!("../../patterns/clock.rle"));
    let mut life_viewer = LifeViewer::new(size().unwrap(), life);

    let stdout = stdout();

    enable_raw_mode().unwrap();

    loop {
        while poll(Duration::from_millis(0)).unwrap() {
            match read().unwrap() {
                Event::Key(key_event) => life_viewer.on_key(key_event),
                Event::Resize(x, y) => life_viewer.resize((x, y)),
                _ => {}
            }
        }

        let mut stdout = stdout.lock();

        stdout.queue(BeginSynchronizedUpdate).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        life_viewer.step(&mut stdout);

        queue!(stdout, MoveTo(0, 0), EndSynchronizedUpdate).unwrap();

        stdout.flush().unwrap();
    }
}
