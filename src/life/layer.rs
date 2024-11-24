use std::collections::HashMap;

use super::{cell::Cell, cell_id::CellId};

#[derive(Debug)]
pub struct Layer {
    cells: Vec<Cell>,
    cells_index_lookup: HashMap<Cell, usize>,
    next_gen: HashMap<CellId, CellId>,
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

    pub fn get_next_gen(&self, cell_id: CellId) -> Option<&CellId> {
        self.next_gen.get(&cell_id)
    }

    pub fn cache_next_gen(&mut self, cell_id: CellId, new_cell_id: CellId) {
        self.next_gen.insert(cell_id, new_cell_id);
    }
}
