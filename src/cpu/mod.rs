pub const GENERAL_REGISTER_SIZE: usize = 16;

pub struct Cpu {
    program_counter: u16,
    index_register: u16,
    general_register: [u8; GENERAL_REGISTER_SIZE]
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            index_register: 0,
            general_register: [0; GENERAL_REGISTER_SIZE]
        }
    }

    pub fn increment_pc(&mut self) -> u16 {
        self.program_counter += 1;
        self.program_counter
    }

    pub fn get_i_register(&self) -> u16 {
        self.index_register
    }

    pub fn set_general_register(&mut self, register: u16, value: u8) {
        self.general_register[register as usize] = value;
    }

    pub fn get_general_register(&mut self, register: u16) -> u8 {
        self.general_register[register as usize]
    }
}