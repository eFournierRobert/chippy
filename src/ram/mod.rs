
pub const PROGRAM_START_ADDRESS: u16 = 0x200;
pub const FONT_START_ADDRESS: u16 = 0x050;

pub struct ram {
    memory: [i8; 4096]
}

impl ram {
    pub fn write(&mut self, address: u16, content: i8) {
        self.memory[address as usize] = content;
    }

    pub fn read(&self, address: u16) -> i8 {
        return self.memory[address as usize];
    }
}