use std::ops::{Index, IndexMut, Range};

const MEMORY_SIZE: usize = 0xFFFF;

pub struct MemoryEdit {
    pub key: u16,
    pub value: u8
}

pub struct MemoryChange {
    pub changes: Vec<MemoryEdit>
}

impl MemoryChange {
    pub fn default() -> MemoryChange {
        MemoryChange { changes: Vec::new() }
    }
}

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

//allows read for Memory[index]
impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

//allows write for Memory[index]
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

    //gets a copy of a range of memory as vector
    pub fn get_slice(&self, range: Range<usize>) -> Vec<u8> {
        self.memory[range].to_vec()
    }

    //map values by bulk to memory, mem_range specifies where in memory
    pub fn memory_map(&mut self, mem_range: Range<usize>, values: Vec<u8>) {
        let mut idx = 0;
        let len = values.len();

        for addr in mem_range {
            self.memory[addr] = values[idx];
            idx += 1;

            if idx >= len {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut memory = Memory::new();

        memory[0x01] = 10;
        assert_eq!(memory[0x01], 10);
    }

    #[test]
    fn test_update() {
        let mut memory = Memory::new();

        memory.update(&MemoryChange {
            changes: vec![MemoryEdit {
                key: 0x01,
                value: 0x0A
            }]
        });

        assert_eq!(memory[0x01], 0x0A);
        assert_eq!(memory[0x02], 0x00);
    }
}