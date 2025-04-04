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

#[derive(Clone, Debug)]
pub struct CompositeCell {
    pub(crate) nw: CellId,
    pub(crate) ne: CellId,
    pub(crate) sw: CellId,
    pub(crate) se: CellId,
    // Ignore for hashing / equality
    pub(crate) next_gen: Option<CellId>,
}

impl Hash for CompositeCell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nw.hash(state);
        self.ne.hash(state);
        self.sw.hash(state);
        self.se.hash(state);
    }
}

impl Eq for CompositeCell {}
impl PartialEq for CompositeCell {
    fn eq(&self, other: &Self) -> bool {
        self.nw == other.nw && self.ne == other.ne && self.sw == other.sw && self.se == other.se
    }
}

impl Cell {
    pub fn composite(nw: CellId, ne: CellId, sw: CellId, se: CellId) -> Self {
        Cell::Composite(CompositeCell {
            nw,
            ne,
            sw,
            se,
            next_gen: None,
        })
    }

    pub fn as_composite(&self) -> &CompositeCell {
        match self {
            Cell::Composite(cell) => cell,
            _ => panic!("Cell is not composite"),
        }
    }

    pub fn layer(&self) -> u8 {
        match self {
            Cell::Base(_) => 0,
            Cell::Composite(CompositeCell { nw, .. }) => nw.layer() as u8 + 1,
        }
    }

    pub fn alive_count(&self) -> usize {
        match self {
            Cell::Base(self::BaseCell::Alive) => 1,
            Cell::Base(self::BaseCell::Dead) => 0,
            Cell::Composite(cell) => {
                cell.nw.alive() + cell.ne.alive() + cell.sw.alive() + cell.se.alive()
            }
        }
    }
}
