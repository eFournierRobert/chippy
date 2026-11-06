/// Address at which programs are loaded.
pub const PROGRAM_START_ADDRESS: u16 = 0x200;
/// Address at which the built-in font is loaded.
pub const FONT_START_ADDRESS: u16 = 0x050;

const RAM_SIZE: usize = 4096;

/// The 4 KiB CHIP-8 memory.
pub struct Ram {
    memory: [u8; RAM_SIZE],
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}

impl Ram {
    /// Creates zero-initialized memory.
    pub fn new() -> Self {
        Self {
            memory: [0; RAM_SIZE],
        }
    }

    /// Writes a byte to memory.
    ///
    /// # Panics
    ///
    /// Panics if `address` is outside memory.
    pub fn write(&mut self, address: u16, content: u8) {
        self.memory[address as usize] = content;
    }

    /// Reads a byte from memory.
    ///
    /// # Panics
    ///
    /// Panics if `address` is outside memory.
    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
