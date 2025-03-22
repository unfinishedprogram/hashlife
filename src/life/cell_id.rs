#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(transparent)]
struct CellIndex(u32);

impl CellIndex {
    pub fn new(layer: u8, index: usize) -> Self {
        CellIndex(((layer as u32) << 24) | index as u32)
    }

    pub fn index(&self) -> usize {
        (self.0 & 0x00ff_ffff) as usize
    }

    pub fn layer(&self) -> usize {
        ((self.0 >> 24) & 0xff) as usize
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
            index: CellIndex::new(layer as u8, index),
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
