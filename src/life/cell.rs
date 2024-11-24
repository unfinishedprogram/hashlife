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
    depth: u8,
    pub(crate) nw: CellId,
    pub(crate) ne: CellId,
    pub(crate) sw: CellId,
    pub(crate) se: CellId,
}

impl Cell {
    pub fn composite(depth: u8, nw: CellId, ne: CellId, sw: CellId, se: CellId) -> Self {
        Cell::Composite(CompositeCell {
            depth,
            nw,
            ne,
            sw,
            se,
        })
    }

    pub fn depth(&self) -> u8 {
        match self {
            Cell::Base(_) => 0,
            Cell::Composite(CompositeCell { depth, .. }) => *depth,
        }
    }
}
