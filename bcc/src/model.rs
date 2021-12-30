#[derive(Debug, Clone, Copy)]
pub struct Cells {
    pub position: usize,
    pub size: usize,
}

impl Cells {
    pub fn position_at(&self, index: usize) -> usize {
        self.position + index
    }

    pub fn last_position(&self) -> usize {
        self.position_at(self.position + self.size - 1)
    }
}

#[derive(Clone, Debug)]
pub struct MemoryModel {
    pub size: usize,
}

impl Default for MemoryModel {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryModel {
    pub fn new() -> Self {
        MemoryModel {
            size: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Cells {
        let position = self.size;
        self.size += size;
        Cells {
            position,
            size,
        }
    }

    pub fn free(&mut self, size: usize) {
        self.size -= size;
    }
}
