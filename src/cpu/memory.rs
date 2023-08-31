use std::ops::{Index, IndexMut, RangeInclusive};

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

    pub fn read_with_range(&self, range: RangeInclusive<usize>) -> Vec<u8> {
        Vec::from(&self.memory[range])
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
    fn test_read_with_range() {
        let mut memory = Memory::new();

        for n in 0x00..=0x05 as u8 {
            memory[n as usize] = n;
        }

        let vec_slice = memory.read_with_range(0x00..=0x05);

        assert_eq!(vec_slice[0x02], 0x02);
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