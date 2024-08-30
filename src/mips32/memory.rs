use crate::errors::{MimicError, MimicErrorType};

#[derive(Debug)]
pub struct Memory {
    pub(crate) text: Vec<u32>,
    pub(crate) data: Vec<u32>,
    blocksize: usize,
    text_start: u32,
    text_end: u32,
    data_start: u32,
    data_end: u32,

    pub(crate) text_generation: u32,
    pub(crate) data_generation: u32,
    blocks_allocated: usize,
}

impl Memory {
    pub fn new_mips_default(blocksize: usize) -> Self {
        let mut mem = Self {
            text: Vec::with_capacity(blocksize),
            data: Vec::with_capacity(blocksize),
            blocksize,
            text_start: 0x00100000, // 0x00400000 / 4
            text_end: 0x04003FFF,   // 0x10010000 / 4 - 1
            data_start: 0x04004000, // 0x10010000 / 4
            data_end: 0x1FFFFFFF,   // 0x80000000 / 4 - 1

            text_generation: 0,
            data_generation: 0,
            blocks_allocated: 2,
        };

        for _ in 0..blocksize {
            mem.text.push(0);
            mem.data.push(0);
        }

        mem
    }

    pub fn load_text(&mut self, text: Vec<u32>) {
        for i in 0..text.len() {
            self.set(self.text_start + i as u32, text[i]);
        }
    }

    pub fn load_data(&mut self, data: Vec<u32>) {
        for i in 0..data.len() {
            self.set(self.data_start + i as u32, data[i]);
        }
    }

    pub fn get(&self, index: u32) -> Result<u32, MimicError> {
        if self.text_start <= index && index <= self.text_end {
            let index = index - self.text_start;

            if index as usize >= self.text.len() {
                return Ok(0);
            }

            unsafe {
                return Ok(*self.text.get_unchecked(index as usize));
            }
        }

        if self.data_start <= index && index <= self.data_end {
            let index = index - self.data_start;

            if index as usize >= self.data.len() {
                return Ok(0);
            }

            unsafe {
                return Ok(*self.data.get_unchecked(index as usize));
            }
        }

        Err(MimicError {
            span: None,
            source: None,
            ty: MimicErrorType::MemoryOutOfBounds{address: index as usize}
        })
    }

    pub fn set(&mut self, index: u32, value: u32) {
        if self.text_start <= index && index <= self.text_end {
            let index = index - self.text_start;

            while index as usize >= self.text.len() {
                self.text.reserve(self.blocksize);
                self.blocks_allocated += 1;
                for _ in 0..self.blocksize {
                    self.text.push(0);
                }
            }

            self.text[index as usize] = value;
            self.text_generation += 1;
        }

        if self.data_start <= index && index <= self.data_end {
            let index = index - self.data_start;

            while index as usize >= self.data.len() {
                self.data.reserve(self.blocksize);
                self.blocks_allocated += 1;
                for _ in 0..self.blocksize {
                    self.data.push(0);
                }
            }

            self.data[index as usize] = value;
            self.data_generation += 1;
        }
    }
}
