mod cell;
mod cell_id;
mod import;
mod layer;
mod pack_unpack;
pub mod print;
mod tree;

use cell::{Cell, CompositeCell};
use cell_id::CellId;
use layer::Layer;

pub struct Life {
    pub root: CellId,
    pub layers: Vec<Layer>,
}

impl Life {
    pub fn new(max_depth: u8) -> Self {
        let mut layers = Vec::new();

        for _ in 0..max_depth {
            layers.push(Layer::new());
        }
        let mut res = Life {
            layers,
            root: CellId::new(0, 0),
        };

        let root = res.empty_of_layer(0);
        res.root = root;
        res
    }

    fn empty_of_layer(&mut self, layer: u8) -> CellId {
        let mut empty_id = self.add_cell(Cell::Base(cell::BaseCell::Dead));
        for layer in 1..=layer {
            empty_id = self.add_cell(Cell::composite(
                layer, empty_id, empty_id, empty_id, empty_id,
            ));
        }
        empty_id
    }

    pub fn add_cell(&mut self, cell: Cell) -> CellId {
        let layer = cell.depth() as usize;
        let index = self.layers[layer].add_cell(cell);

        CellId::new(layer, index)
    }

    pub fn get_cell(&self, cell_id: CellId) -> Option<&Cell> {
        self.layers.get(cell_id.layer())?.get_cell(cell_id.index())
    }

    pub fn step(&mut self) {}

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

        assert_eq!(life.get_cell(empty_0).unwrap().depth(), 0);
        assert_eq!(life.get_cell(empty_1).unwrap().depth(), 1);
        assert_eq!(life.get_cell(empty_2).unwrap().depth(), 2);
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
