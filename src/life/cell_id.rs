#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct CellId {
    layer: usize,
    index: usize,
}

impl CellId {
    pub fn new(layer: usize, index: usize) -> Self {
        CellId { layer, index }
    }

    pub fn layer(&self) -> usize {
        self.layer
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
