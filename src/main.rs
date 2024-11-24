use life::{import::rle_to_cell_positions, print::print_positions, Life};

mod life;

fn main() {
    println!("Starting");
    let pattern = rle_to_cell_positions(include_str!("./life.rle").to_string(), 0, 0);
    println!("Converted pattern");
    let mut life = Life::from_cell_positions(32, pattern);

    for _ in 0..10000 {
        println!("Stepping");

        life.step();
        print!("{}[2J", 27 as char);
        print_positions(life.cell_positions());
        println!("Alive: {}", life.root.alive());
        life.print_stats();
    }
}
