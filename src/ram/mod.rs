
pub const PROGRAM_START_ADDRESS: u16 = 0x200;
pub const FONT_START_ADDRESS: u16 = 0x050;

pub const RAM_SIZE: usize = 4096;

pub struct Ram {
    memory: [u8; RAM_SIZE]
}

impl Ram {
    pub fn new() -> Self {
        Self {
            memory: [0; RAM_SIZE]
        }
    }

    pub fn write(&mut self, address: u16, content: u8) {
        self.memory[address as usize] = content;
    }

    pub fn read(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }
}