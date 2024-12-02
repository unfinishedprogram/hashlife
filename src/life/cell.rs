use std::hash::Hash;

use super::cell_id::CellId;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Cell {
    Base(BaseCell),
    Composite(CompositeCell),
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum BaseCell {
    Alive,
    Dead,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct CompositeCell {
    pub(crate) nw: CellId,
    pub(crate) ne: CellId,
    pub(crate) sw: CellId,
    pub(crate) se: CellId,
}

impl Cell {
    pub fn composite(nw: CellId, ne: CellId, sw: CellId, se: CellId) -> Self {
        Cell::Composite(CompositeCell { nw, ne, sw, se })
    }

    pub fn as_composite(&self) -> &CompositeCell {
        match self {
            Cell::Composite(cell) => cell,
            _ => panic!("Cell is not composite"),
        }
    }

    pub fn depth(&self) -> u8 {
        match self {
            Cell::Base(_) => 0,
            Cell::Composite(CompositeCell { nw, .. }) => nw.layer() as u8 + 1,
        }
    }
}
