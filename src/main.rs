use life::{import::rle_to_cell_positions, print::print_positions, Life};

mod life;

fn main() {
    println!("Starting");
    let pattern = rle_to_cell_positions(include_str!("./life.rle").to_string(), 0, 0);
    println!("Converted pattern");
    let mut life = Life::from_cell_positions(32, pattern);

    loop {
        println!("Stepping");
        let start = std::time::Instant::now();
        life.step();
        print!("{}[2J", 27 as char);
        print_positions(life.cell_positions(5));
        println!("Alive: {}", life.root.alive());
        println!("Step time: {:?}", start.elapsed());
        life.print_stats();
    }
}
