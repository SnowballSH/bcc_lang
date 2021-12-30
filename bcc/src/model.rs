use std::collections::BTreeMap;

pub type Id = usize;

#[derive(Debug, Clone, Copy)]
pub struct Cells {
    pub position: usize,
    pub size: usize,
}

impl Cells {
    pub fn position_at(&self, index: usize) -> usize {
        self.position + index
    }
}

#[derive(Clone, Debug)]
pub struct MemoryModel {
    memory: BTreeMap<Id, Cells>,
    pub size: usize,
}

impl MemoryModel {
    pub fn new() -> Self {
        MemoryModel {
            memory: BTreeMap::new(),
            size: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> usize {
        let position = self.size;
        self.size += size;
        position
    }

    pub fn store_value(&mut self, id: Id, size: usize) {
        let position = self.allocate(size);

        let cells = Cells {
            position,
            size,
        };

        self.memory.insert(id, cells);
    }
}
