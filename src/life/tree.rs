use crate::life::cell::Cell;

use super::{cell_id::CellId, Life};

impl Life {
    pub fn join(&mut self, nw: CellId, ne: CellId, sw: CellId, se: CellId) -> CellId {
        debug_assert_eq!(nw.layer(), ne.layer());
        debug_assert_eq!(nw.layer(), sw.layer());
        debug_assert_eq!(nw.layer(), se.layer());

        let new_node_layer = nw.layer() + 1;

        self.add_cell(Cell::composite(new_node_layer as u8, nw, ne, sw, se))
    }

    pub fn centered_subnode(&mut self, id: CellId) {
        todo!()
    }
}
