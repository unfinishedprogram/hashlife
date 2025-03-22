use std::collections::HashMap;

use super::{cell::Cell, cell_id::CellId};

#[derive(Clone, Debug, Default)]
pub struct Layer {
    pub calls: usize,
    cells: Vec<Cell>,
    cells_index_lookup: HashMap<Cell, usize>,
}

impl Layer {
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn add_cell(&mut self, cell: Cell) -> usize {
        self.calls += 1;
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
        self.cells[cell_id.index()].as_composite().next_gen.as_ref()
    }

    pub fn cache_next_gen(&mut self, cell_id: CellId, new_cell_id: CellId) {
        if let Some(Cell::Composite(cell)) = self.cells.get_mut(cell_id.index()) {
            cell.next_gen = Some(new_cell_id)
        }
    }
}
