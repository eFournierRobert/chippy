use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::input::Keyboard;
use crate::{fonts, ram};
use crate::ram::Ram;
use crate::stack::Stack;
use crate::timers::Timers;

pub struct Interpreter {
    cpu: Cpu,
    display: Display,
    keyboard: Keyboard,
    ram: Ram,
    stack: Stack,
    timers: Timers
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            cpu: Cpu::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            ram: Ram::new(),
            stack: Stack::new(),
            timers: Timers::new()
        };

        for (i, byte) in fonts::FONT.iter().flatten().enumerate() {
            interpreter.ram.write(ram::FONT_START_ADDRESS + i as u16, *byte);
        }

        interpreter
    }

    pub fn load_program(&mut self, path: PathBuf) -> Result<(), Error>{
        let mut f = File::open(path)?;
        let mut bytes = Vec::new();

        f.read_to_end(&mut bytes)?;

        for (i, byte) in bytes.iter().enumerate() {
            self.ram.write(ram::PROGRAM_START_ADDRESS + i as u16, *byte);
        }

        self.cpu.set_pc(ram::PROGRAM_START_ADDRESS);

        Ok(())
    }

    fn fetch_instruction(&mut self) -> u16 {
        let opcode = u16::from_be_bytes([
            self.ram.read(self.cpu.get_pc()),
            self.ram.read(self.cpu.get_pc() + 1)
        ]);

        self.cpu.increment_pc();

        opcode
    }

    fn decode_instruction(instruction: u16) -> [u16; 4] {
        let n1 = (instruction >> 12) & 0xF;
        let n2 = (instruction >> 8) & 0xF;
        let n3 = (instruction >> 4) & 0xF;
        let n4 = instruction & 0xF;

        [n1, n2, n3, n4]
    }
}