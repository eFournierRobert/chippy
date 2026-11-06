/// Number of general-purpose registers, including the flag register `VF`.
pub const GENERAL_REGISTER_SIZE: usize = 16;

/// CHIP-8 program counter, index register, and general-purpose registers.
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
    /// Creates a CPU with all registers set to zero.
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            index_register: 0,
            general_register: [0; GENERAL_REGISTER_SIZE],
        }
    }

    /// Advances the program counter by one two-byte instruction.
    pub fn increment_pc(&mut self) {
        self.program_counter += 2;
    }

    /// Returns the program counter.
    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    /// Sets the program counter.
    pub fn set_pc(&mut self, value: u16) {
        self.program_counter = value;
    }

    /// Returns the index register.
    pub fn get_i_register(&self) -> u16 {
        self.index_register
    }

    /// Sets the index register.
    pub fn set_i_register(&mut self, value: u16) {
        self.index_register = value;
    }

    /// Sets a general-purpose register.
    ///
    /// # Panics
    ///
    /// Panics if `register` is outside `0..16`.
    pub fn set_general_register(&mut self, register: u16, value: u8) {
        self.general_register[register as usize] = value;
    }

    /// Returns a general-purpose register.
    ///
    /// # Panics
    ///
    /// Panics if `register` is outside `0..16`.
    pub fn get_general_register(&mut self, register: u16) -> u8 {
        self.general_register[register as usize]
    }

    /// Adds a value to a register, wrapping on overflow without changing `VF`.
    ///
    /// # Panics
    ///
    /// Panics if `register` is outside `0..16`.
    pub fn add_value_to_general_register(&mut self, register: u16, value: u8) {
        self.general_register[register as usize] =
            self.general_register[register as usize].wrapping_add(value);
    }

    /// Adds `Vy` to `Vx`, storing the carry in `VF`.
    ///
    /// # Panics
    ///
    /// Panics if either register index is outside `0..16`.
    pub fn add_registers(&mut self, x: u16, y: u16) {
        let (result, carry) =
            self.general_register[x as usize].overflowing_add(self.general_register[y as usize]);

        self.general_register[x as usize] = result;
        self.general_register[0xF] = carry as u8;
    }

    /// Subtracts `value` from `register`, storing the result in `store_reg` or `register`.
    ///
    /// Sets `VF` to 1 when there is no borrow and 0 otherwise.
    ///
    /// # Panics
    ///
    /// Panics if a register index is outside `0..16`.
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

    /// Shifts a register right by one and stores the discarded bit in `VF`.
    ///
    /// # Panics
    ///
    /// Panics if `register` is outside `0..16`.
    pub fn shift_right(&mut self, register: u16) {
        let value = self.general_register[register as usize];

        self.general_register[register as usize] = value >> 1;

        self.general_register[0xF] = value & 1;
    }

    /// Shifts a register left by one and stores the discarded bit in `VF`.
    ///
    /// # Panics
    ///
    /// Panics if `register` is outside `0..16`.
    pub fn shift_left(&mut self, register: u16) {
        let value = self.general_register[register as usize];
        self.general_register[register as usize] = value << 1;

        self.general_register[0xF] = (value >> 7) & 1;
    }
}
