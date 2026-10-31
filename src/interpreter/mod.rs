use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::input::Keyboard;
use crate::{display, fonts, ram};
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

    pub fn start_execution(&mut self) {
        let mut instruction = self.fetch_instruction();
        while instruction != 0 {
            let decoded_instruction = self.decode_instruction(instruction);

            match decoded_instruction[0] {
                0x0 => {
                    if decoded_instruction[1] == 0 &&
                        decoded_instruction[2] == 0xE &&
                        decoded_instruction[3] == 0 {

                        self.display.clear_display();
                        self.display.print_display();
                    }
                }
                0x1 => {
                    self.cpu.set_pc(instruction & 0x0FFF);
                }
                0x6 => {
                    self.cpu.set_general_register(
                        decoded_instruction[1],
                        (instruction & 0x00FF) as u8
                    );
                }
                0x7 => {
                    self.cpu.add_value_to_general_register(
                        decoded_instruction[1],
                        (instruction & 0x00FF) as u8
                    );
                }
                0xA => {
                    self.cpu.set_i_register(instruction & 0x0FFF);
                }
                0xD => {
                    self.cpu.set_general_register(0xF, false as u8);

                    let x = self.cpu.get_general_register(decoded_instruction[1]);
                    let y = self.cpu.get_general_register(decoded_instruction[2]);
                    let height = decoded_instruction[3];
                    let sprite: Vec<u8> = (0..height)
                        .map(|row| self.ram.read(self.cpu.get_i_register() + row))
                        .collect();

                    if self.display.draw_sprite(x as usize, y as usize, sprite) {
                        self.cpu.set_general_register(0xF, true as u8);
                    }

                    self.display.print_display();
                }
                _ => {
                    println!("unsupported instruction: {:?}", decoded_instruction);
                }
            }

            instruction = self.fetch_instruction();
            sleep(Duration::from_millis(100));
        }
    }

    fn fetch_instruction(&mut self) -> u16 {
        let opcode = u16::from_be_bytes([
            self.ram.read(self.cpu.get_pc()),
            self.ram.read(self.cpu.get_pc() + 1)
        ]);

        self.cpu.increment_pc();

        opcode
    }

    fn decode_instruction(&self, instruction: u16) -> [u16; 4] {
        let n1 = (instruction >> 12) & 0xF;
        let n2 = (instruction >> 8) & 0xF;
        let n3 = (instruction >> 4) & 0xF;
        let n4 = instruction & 0xF;

        [n1, n2, n3, n4]
    }
}