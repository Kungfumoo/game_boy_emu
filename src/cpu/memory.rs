use std::ops::{Index, IndexMut};
use super::instructions::MemoryChange;

pub const MEMORY_SIZE: usize = 65536;

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORY_SIZE]
        }
    }

    pub fn update(&mut self, change: &MemoryChange) {
        for mem_change in change.changes.iter() {
            self[mem_change.key as usize] = mem_change.value;
        }
    }
}