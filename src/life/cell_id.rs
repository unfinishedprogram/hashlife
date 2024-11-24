#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct CellId {
    layer: usize,
    index: usize,
    alive: usize,
}

impl CellId {
    pub fn new(layer: usize, index: usize, alive: usize) -> Self {
        CellId {
            layer,
            index,
            alive,
        }
    }

    pub fn layer(&self) -> usize {
        self.layer
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn alive(&self) -> usize {
        self.alive
    }
}
