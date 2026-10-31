
pub const PROGRAM_START_ADDRESS: usize = 0x200;
pub const FONT_START_ADDRESS: usize = 0x050;

pub struct ram {
    memory: [i8; 4096]
}

impl ram {
    pub fn write(&mut self, address: usize, content: i8) {
        self.memory[address] = content;
    }

    pub fn read(&self, address: usize) -> i8 {
        return self.memory[address];
    }
}