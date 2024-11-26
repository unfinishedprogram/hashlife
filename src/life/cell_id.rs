#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(transparent)]
struct CellIndex(usize);

impl CellIndex {
    pub fn new(layer: u16, index: usize) -> Self {
        CellIndex(((layer as usize) << 48) | index)
    }

    pub fn index(&self) -> usize {
        self.0 & 0x0000_ffff_ffff_ffff
    }

    pub fn layer(&self) -> usize {
        (self.0 >> 48) & 0xffff
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct CellId {
    alive: usize,
    index: CellIndex,
}

impl CellId {
    pub fn new(layer: usize, index: usize, alive: usize) -> Self {
        CellId {
            index: CellIndex::new(layer as u16, index),
            alive,
        }
    }

    pub fn layer(&self) -> usize {
        self.index.layer()
    }

    pub fn index(&self) -> usize {
        self.index.index()
    }

    pub fn alive(&self) -> usize {
        self.alive
    }
}
