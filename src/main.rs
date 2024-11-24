use life::{print::print_positions, Life};

mod life;

fn main() {
    // Acorn
    let life = Life::from_cell_positions(
        8,
        vec![(0, 0), (1, 0), (1, 2), (3, 1), (4, 0), (5, 0), (6, 0)],
    );
    print_positions(life.cell_positions());
}
