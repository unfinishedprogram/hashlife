use std::collections::{HashMap, HashSet};

use super::cell::Cell;

pub struct Layer {
    cells: Vec<Cell>,
    cells_index_lookup: HashMap<Cell, usize>,
    next_gen: HashMap<Cell, Cell>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            cells: vec![],
            cells_index_lookup: HashMap::new(),
            next_gen: HashMap::new(),
        }
    }

    pub fn add_cell(&mut self, cell: Cell) -> usize {
        if let Some(index) = self.cells_index_lookup.get(&cell) {
            return *index;
        }

        let cell_index = self.cells.len();
        self.cells_index_lookup.insert(cell.clone(), cell_index);
        self.cells.push(cell);

        cell_index
    }

    pub fn get_cell(&self, index: usize) -> Option<&Cell> {
        self.cells.get(index)
    }
}
