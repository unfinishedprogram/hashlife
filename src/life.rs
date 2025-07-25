mod cell;
mod cell_id;
pub mod import;
mod layer;
pub mod pack_unpack;
mod tree;

use cell::Cell;
use cell_id::CellId;
use layer::Layer;

#[derive(Clone)]
pub struct Life {
    pub root: CellId,
    pub layers: Vec<Layer>,
    pub base_alive: CellId,
    pub base_dead: CellId,
}

impl Life {
    pub fn new(max_depth: u8) -> Self {
        let mut layers = Vec::new();

        for _ in 0..max_depth {
            layers.push(Layer::default());
        }

        let base_alive = Cell::Base(cell::BaseCell::Alive);
        let base_dead = Cell::Base(cell::BaseCell::Dead);
        let base_alive = CellId::new(0, layers[0].add_cell(base_alive), 1);
        let base_dead = CellId::new(0, layers[0].add_cell(base_dead), 0);

        let mut res = Life {
            layers,
            root: CellId::new(0, 0, 0),
            base_alive,
            base_dead,
        };

        let root = res.empty_of_layer(0);
        res.root = root;
        res
    }

    pub fn from_rle(rle: &str) -> Self {
        let pattern = import::rle_to_cell_positions(rle.to_string(), 0, 0);
        Life::from_cell_positions(32, pattern)
    }

    pub fn print_stats(&self) {
        let mut total = 0;
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.size() == 0 {
                break;
            }
            total += layer.size();
            println!("Layer {}: {}", i, layer.size());
        }
        println!("Total: {total}");
    }

    fn empty_of_layer(&mut self, layer: u8) -> CellId {
        let mut empty_id = self.add_cell(Cell::Base(cell::BaseCell::Dead));
        for _ in 1..=layer {
            empty_id = self.add_cell(Cell::composite(empty_id, empty_id, empty_id, empty_id));
        }
        empty_id
    }

    pub fn add_cell(&mut self, cell: Cell) -> CellId {
        let layer = cell.layer() as usize;
        let alive = cell.alive_count();
        let index = self.layers[layer].add_cell(cell);
        CellId::new(layer, index, alive)
    }

    pub fn get_cell(&self, cell_id: CellId) -> Option<&Cell> {
        self.layers.get(cell_id.layer())?.get_cell(cell_id.index())
    }

    pub fn step(&mut self) {
        while !self.is_padded(self.root) {
            self.root = self.padded(self.root);
        }
        self.root = self.padded(self.root);

        self.root = self.next_generation(self.root);
    }

    pub fn padded(&mut self, cell_id: CellId) -> CellId {
        if let Cell::Composite(cell) = self.get_cell(cell_id).unwrap().clone() {
            let empty = self.empty_of_layer((cell_id.layer() - 1) as u8);

            let nw = self.join(empty, empty, empty, cell.nw);
            let ne = self.join(empty, empty, cell.ne, empty);
            let sw = self.join(empty, cell.sw, empty, empty);
            let se = self.join(cell.se, empty, empty, empty);

            self.join(nw, ne, sw, se)
        } else {
            let empty = self.empty_of_layer(0);
            self.join(cell_id, empty, empty, empty)
        }
    }

    fn is_padded(&mut self, cell_id: CellId) -> bool {
        let total = cell_id.alive();
        let center = self.centered_subnode(cell_id).alive();
        total - center == 0
    }

    pub fn next_generation(&mut self, cell_id: CellId) -> CellId {
        if cell_id.layer() < 2 {
            unreachable!("Next generation should never be called with a base layer cell");
        }

        if let Some(next_gen_id) = self.layers[cell_id.layer()].get_next_gen(cell_id) {
            return *next_gen_id;
        }

        if cell_id.layer() == 2 {
            let cell = self.get_cell(cell_id).unwrap().as_composite();
            let res = self.next_generation_base_case(cell.nw, cell.ne, cell.sw, cell.se);
            self.layers[cell_id.layer()].cache_next_gen(cell_id, res);
            return res;
        }

        let cell = self.get_cell(cell_id).unwrap().as_composite();

        let nw = cell.nw;
        let ne = cell.ne;
        let sw = cell.sw;
        let se = cell.se;

        let n00 = self.centered_subnode(nw);
        let n01 = self.centered_horizontal(nw, ne);
        let n02 = self.centered_subnode(ne);
        let n10 = self.centered_vertical(nw, sw);
        let n11 = self.centered_sub_subnode(nw, ne, sw, se);
        let n12 = self.centered_vertical(ne, se);
        let n20 = self.centered_subnode(sw);
        let n21 = self.centered_horizontal(sw, se);
        let n22 = self.centered_subnode(se);

        let nw = self.join(n00, n01, n10, n11);
        let ne = self.join(n01, n02, n11, n12);
        let sw = self.join(n10, n11, n20, n21);
        let se = self.join(n11, n12, n21, n22);

        let nw = self.next_generation(nw);
        let ne = self.next_generation(ne);
        let sw = self.next_generation(sw);
        let se = self.next_generation(se);

        let res = self.join(nw, ne, sw, se);

        assert_eq!(res.layer(), cell_id.layer() - 1);

        self.layers[cell_id.layer()].cache_next_gen(cell_id, res);
        res
    }

    // Takes 4 2x2 nodes and returns the new 2x2 center node
    fn next_generation_base_case(
        &mut self,
        nw: CellId,
        ne: CellId,
        sw: CellId,
        se: CellId,
    ) -> CellId {
        debug_assert_eq!(nw.layer(), 1);
        debug_assert_eq!(ne.layer(), 1);
        debug_assert_eq!(sw.layer(), 1);
        debug_assert_eq!(se.layer(), 1);

        let base_alive = self.base_alive;
        let base_dead = self.base_dead;

        let nw = self.get_cell(nw).unwrap().as_composite();
        let ne = self.get_cell(ne).unwrap().as_composite();
        let sw = self.get_cell(sw).unwrap().as_composite();
        let se = self.get_cell(se).unwrap().as_composite();

        let cells = [
            [
                nw.nw.alive() > 0,
                nw.ne.alive() > 0,
                ne.nw.alive() > 0,
                ne.ne.alive() > 0,
            ],
            [
                nw.sw.alive() > 0,
                nw.se.alive() > 0,
                ne.sw.alive() > 0,
                ne.se.alive() > 0,
            ],
            [
                sw.nw.alive() > 0,
                sw.ne.alive() > 0,
                se.nw.alive() > 0,
                se.ne.alive() > 0,
            ],
            [
                sw.sw.alive() > 0,
                sw.se.alive() > 0,
                se.sw.alive() > 0,
                se.se.alive() > 0,
            ],
        ];

        let [nw, ne, sw, se] = [(1, 1), (2, 1), (1, 2), (2, 2)].map(|(x, y)| {
            let alive = cells[y][x];

            let directions: [(i32, i32); 8] = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];

            let surrounding: u32 = directions
                .map(|(dx, dy)| cells[(y as i32 + dy) as usize][(x as i32 + dx) as usize] as u32)
                .iter()
                .sum();

            if surrounding == 3 || alive && surrounding == 2 {
                base_alive
            } else {
                base_dead
            }
        });

        self.join(nw, ne, sw, se)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_cell_can_be_retrieved_via_id() {
        let mut life = Life::new(8);

        let cell_alive = Cell::Base(cell::BaseCell::Alive);
        let cell_dead = Cell::Base(cell::BaseCell::Dead);

        let alive_id = life.add_cell(cell_alive.clone());
        let dead_id = life.add_cell(cell_dead.clone());

        let alive = life.get_cell(alive_id).unwrap();
        let dead = life.get_cell(dead_id).unwrap();

        assert_eq!(alive, &cell_alive);
        assert_eq!(dead, &cell_dead);
    }

    #[test]
    fn empty_of_layer_works() {
        let mut life = Life::new(8);

        let empty_0 = life.empty_of_layer(0);
        let empty_1 = life.empty_of_layer(1);
        let empty_2 = life.empty_of_layer(2);

        assert_eq!(life.get_cell(empty_0).unwrap().layer(), 0);
        assert_eq!(life.get_cell(empty_1).unwrap().layer(), 1);
        assert_eq!(life.get_cell(empty_2).unwrap().layer(), 2);
    }

    #[test]
    fn padding_works() {
        let mut life = Life::new(8);

        let empty = life.empty_of_layer(0);
        assert_eq!(empty.layer(), 0);
        let padded_1 = life.padded(empty);
        assert_eq!(padded_1.layer(), 1);
        let padded_2 = life.padded(padded_1);
        assert_eq!(padded_2.layer(), 2);
    }
}
