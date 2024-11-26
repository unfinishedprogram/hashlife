use std::io::{stdout, Write};

use hashlife::life::{print::print_positions, Life};

fn main() {
    println!("Starting");

    let mut life = Life::from_rle(include_str!("../../patterns/clock.rle"));
    let clear_char = "\x1b[2J\x1b[1;1H".to_string();

    loop {
        let start = std::time::Instant::now();
        life.step();
        let deep = 10;
        let depth = life.root.layer().max(deep) - deep;

        let mut str = clear_char.clone() + &print_positions(life.cell_positions(depth as u8));
        str.push_str(&format!("Alive: {}\n", life.root.alive()));
        str.push_str(&format!("Step time: {:?}\n", start.elapsed()));
        print!("{}", str);
        stdout().flush().unwrap()
    }
}
