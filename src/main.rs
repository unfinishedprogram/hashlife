use std::io::{stdout, Write};

use life::{import::rle_to_cell_positions, print::print_positions, Life};

mod life;

fn main() {
    println!("Starting");
    let pattern = rle_to_cell_positions(include_str!("./life.rle").to_string(), 0, 0);
    println!("Converted pattern");

    let mut life = Life::from_cell_positions(32, pattern);
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
