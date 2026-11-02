pub const GENERAL_REGISTER_SIZE: usize = 16;

pub struct Cpu {
    program_counter: u16,
    index_register: u16,
    general_register: [u8; GENERAL_REGISTER_SIZE],
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            index_register: 0,
            general_register: [0; GENERAL_REGISTER_SIZE],
        }
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += 2;
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn set_pc(&mut self, value: u16) {
        self.program_counter = value;
    }

    pub fn get_i_register(&self) -> u16 {
        self.index_register
    }

    pub fn set_i_register(&mut self, value: u16) {
        self.index_register = value;
    }

    pub fn set_general_register(&mut self, register: u16, value: u8) {
        self.general_register[register as usize] = value;
    }

    pub fn get_general_register(&mut self, register: u16) -> u8 {
        self.general_register[register as usize]
    }

    pub fn add_value_to_general_register(&mut self, register: u16, value: u8) {
        self.general_register[register as usize] =
            self.general_register[register as usize].wrapping_add(value);
    }

    pub fn add_registers(&mut self, x: u16, y: u16) {
        let (result, carry) =
            self.general_register[x as usize].overflowing_add(self.general_register[y as usize]);

        self.general_register[x as usize] = result;
        self.general_register[0xF] = carry as u8;
    }

    pub fn subtract_value_to_general_register(
        &mut self,
        register: u16,
        value: u8,
        store_reg: Option<u16>,
    ) {
        let (result, carry) = self.general_register[register as usize].overflowing_sub(value);

        if let Some(reg) = store_reg {
            self.general_register[reg as usize] = result;
        } else {
            self.general_register[register as usize] = result;
        }
        self.general_register[0xF] = !carry as u8;
    }

    pub fn shift_right(&mut self, register: u16) {
        let value = self.general_register[register as usize];

        self.general_register[register as usize] = value >> 1;

        self.general_register[0xF] = value & 1;
    }

    pub fn shift_left(&mut self, register: u16) {
        let value = self.general_register[register as usize];
        self.general_register[register as usize] = value << 1;

        self.general_register[0xF] = (value >> 7) & 1;
    }
}
