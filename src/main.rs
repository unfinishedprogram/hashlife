use life::{
    print::{print_positions, print_positions_brail},
    Life,
};

mod life;

fn main() {
    let acorn = vec![(0, 0), (1, 0), (1, 2), (3, 1), (4, 0), (5, 0), (6, 0)];
    let still = vec![
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (2, 2),
        (2, 3),
        (3, 2),
        (3, 3),
    ];

    // Acorn
    let mut life = Life::from_cell_positions(32, still);

    // dbg!(&life.layers);

    for i in 0..100 {
        life.step();
        print_positions_brail(life.cell_positions());
    }
}
