use crate::cpu::Cpu;
use crate::display::Display;
use crate::input::Keyboard;
use crate::ram::Ram;
use crate::stack::Stack;
use crate::timers::Timers;
use crate::{fonts, ram};
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::terminal_guard::TerminalGuard;

const CPU_FREQUENCY: u64 = 500;
const CYCLE_DURATION: Duration = Duration::from_nanos(1_000_000_000 / CPU_FREQUENCY);

/// A CHIP-8 interpreter with terminal display and keyboard input.
pub struct Interpreter {
    cpu: Cpu,
    display: Display,
    keyboard: Arc<Mutex<Keyboard>>,
    ram: Ram,
    stack: Stack,
    timers: Timers,
    quit: Arc<AtomicBool>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    /// Creates an interpreter and loads the built-in font into memory.
    pub fn new() -> Self {
        let mut interpreter = Self {
            cpu: Cpu::default(),
            display: Display::default(),
            keyboard: Arc::new(Mutex::new(Keyboard::default())),
            ram: Ram::new(),
            stack: Stack::default(),
            timers: Timers::default(),
            quit: Arc::new(AtomicBool::new(false)),
        };

        for (i, byte) in fonts::FONT.iter().flatten().enumerate() {
            interpreter
                .ram
                .write(ram::FONT_START_ADDRESS + i as u16, *byte);
        }

        interpreter
    }

    /// Loads a program into memory and sets the program counter to its start.
    ///
    /// # Errors
    ///
    /// Returns an error if the program file cannot be opened or read.
    ///
    /// # Panics
    ///
    /// Panics if the program does not fit in memory.
    pub fn load_program(&mut self, path: PathBuf) -> Result<(), Error> {
        let mut f = File::open(path)?;
        let mut bytes = Vec::new();

        f.read_to_end(&mut bytes)?;

        if bytes.len() > ram::RAM_SIZE {
            eprintln!("Binary is too big to be loaded in memory");
            exit(1);
        }

        for (i, byte) in bytes.iter().enumerate() {
            self.ram.write(ram::PROGRAM_START_ADDRESS + i as u16, *byte);
        }

        self.cpu.set_pc(ram::PROGRAM_START_ADDRESS);

        Ok(())
    }

    /// Runs the loaded program until a zero instruction or a quit request.
    ///
    /// Initializes terminal input and display; Ctrl-C requests termination.
    ///
    /// # Panics
    ///
    /// Panics on terminal setup or output errors, invalid memory access, or an empty return stack.
    pub fn start_execution(&mut self) {
        let _terminal = TerminalGuard::new().expect("can't initialize terminal");

        self.start_keyboard_input();

        let mut instruction = self.fetch_instruction();
        while instruction != 0 && !self.quit.load(Ordering::Relaxed) {
            let start = Instant::now();
            self.timers.tick();

            self.execute_instruction(instruction);

            if self.quit.load(Ordering::Relaxed) {
                break;
            }
            instruction = self.fetch_instruction();

            let elapsed = start.elapsed();
            if elapsed < CYCLE_DURATION {
                thread::sleep(CYCLE_DURATION - elapsed);
            }
        }
    }

    /// Starts a thread to update keyboard state from terminal events.
    fn start_keyboard_input(&mut self) {
        let input_keyboard = Arc::clone(&self.keyboard);
        let input_quit = Arc::clone(&self.quit);

        thread::spawn(move || {
            loop {
                let (key, kind) = match Keyboard::poll_keyboard() {
                    Ok(Some(event)) => event,
                    Ok(None) => {
                        input_quit.store(true, Ordering::Relaxed);
                        break;
                    }
                    Err(error) => {
                        eprintln!("keyboard input error: {error}");
                        break;
                    }
                };

                let mut keyboard = input_keyboard.lock().unwrap();
                match kind {
                    crossterm::event::KeyEventKind::Press
                    | crossterm::event::KeyEventKind::Repeat => keyboard.press(key),
                    crossterm::event::KeyEventKind::Release => keyboard.release(key),
                }
            }
        });
    }

    /// Decodes and executes one instruction.
    fn execute_instruction(&mut self, instruction: u16) {
        let [op, x, y, n] = self.decode_instruction(instruction);

        match op {
            0x0 => {
                if x == 0 && y == 0xE && n == 0 {
                    self.display.clear_display();
                    self.display.print_display();
                }

                if x == 0 && y == 0xE && n == 0xE {
                    let new_pc = self.stack.pop().expect("cannot pop empty stack");
                    self.cpu.set_pc(new_pc);
                }
            }
            0x1 => {
                self.cpu.set_pc(instruction & 0x0FFF);
            }
            0x6 => {
                self.cpu
                    .set_general_register(x, (instruction & 0x00FF) as u8);
            }
            0x7 => {
                self.cpu
                    .add_value_to_general_register(x, (instruction & 0x00FF) as u8);
            }
            0xA => {
                self.cpu.set_i_register(instruction & 0x0FFF);
            }
            0xD => {
                self.cpu.set_general_register(0xF, false as u8);

                let x = self.cpu.get_general_register(x);
                let y = self.cpu.get_general_register(y);
                let height = n;
                let sprite: Vec<u8> = (0..height)
                    .map(|row| self.ram.read(self.cpu.get_i_register() + row))
                    .collect();

                if self.display.draw_sprite(x as usize, y as usize, sprite) {
                    self.cpu.set_general_register(0xF, true as u8);
                }

                self.display.print_display();
            }
            0x2 => {
                self.stack.push(self.cpu.get_pc());
                self.cpu.set_pc(instruction & 0x0FFF);
            }
            0x3 => {
                let addr = x;
                let value = instruction & 0x00FF;

                if self.cpu.get_general_register(addr) == value as u8 {
                    self.cpu.increment_pc();
                }
            }
            0x4 => {
                let addr = x;
                let value = instruction & 0x00FF;

                if self.cpu.get_general_register(addr) != value as u8 {
                    self.cpu.increment_pc();
                }
            }
            0x5 => {
                let first_addr = x;
                let second_addr = y;

                if self.cpu.get_general_register(first_addr)
                    == self.cpu.get_general_register(second_addr)
                {
                    self.cpu.increment_pc();
                }
            }
            0x9 => {
                let first_addr = x;
                let second_addr = y;

                if self.cpu.get_general_register(first_addr)
                    != self.cpu.get_general_register(second_addr)
                {
                    self.cpu.increment_pc();
                }
            }
            0x8 => {
                self.execute_arithmetic_ops(x, y, n);
            }
            0xB => {
                let v0_value = self.cpu.get_general_register(0x0);
                self.cpu.set_pc(v0_value as u16 + (instruction & 0x0FFF));
            }
            0xC => {
                let mut value: u8 = rand::random();
                value &= instruction as u8;
                self.cpu.set_general_register(x, value);
            }
            0xF => {
                self.execute_f_instruction(x, instruction & 0x00FF);
            }
            0xE => {
                let code = instruction & 0x00FF;
                let vx_value = self.cpu.get_general_register(x);

                self.execute_skip_if_key(vx_value, code, instruction);
            }
            _ => {
                println!("unsupported instruction: {instruction:#06X}");
            }
        }
    }

    /// Executes an `8xyN` register operation.
    fn execute_arithmetic_ops(&mut self, vx: u16, vy: u16, operation: u16) {
        let vx_value = self.cpu.get_general_register(vx);
        let vy_value = self.cpu.get_general_register(vy);

        match operation {
            0x0 => {
                self.cpu.set_general_register(vx, vy_value);
            }
            0x1 => {
                self.cpu.set_general_register(vx, vx_value | vy_value);
            }
            0x2 => {
                self.cpu.set_general_register(vx, vx_value & vy_value);
            }
            0x3 => {
                self.cpu.set_general_register(vx, vx_value ^ vy_value);
            }
            0x4 => {
                self.cpu.add_registers(vx, vy);
            }
            0x5 => self
                .cpu
                .subtract_value_to_general_register(vx, vy_value, None),
            0x7 => self
                .cpu
                .subtract_value_to_general_register(vy, vx_value, Some(vx)),
            0x6 => self.cpu.shift_right(vx),
            0xE => self.cpu.shift_left(vx),
            _ => println!("unsupported arithmetic instruction: {operation:#X}"),
        }
    }

    /// Executes a keyboard-dependent skip instruction.
    fn execute_skip_if_key(&mut self, vx_value: u8, code: u16, instruction: u16) {
        match code {
            0x9E => {
                if self.keyboard.lock().unwrap().is_pressed(vx_value as usize) {
                    self.cpu.increment_pc();
                }
            }
            0xA1 if !self.keyboard.lock().unwrap().is_pressed(vx_value as usize) => {
                self.cpu.increment_pc();
            }
            0xA1 => {}
            _ => println!("unsupported instruction: {instruction:#06X}"),
        }
    }

    /// Waits for a key press and release while updating timers.
    fn execute_blocking_get_key(&mut self, register: u16) {
        let mut pressed_key = None;

        while !self.quit.load(Ordering::Relaxed) {
            self.timers.tick();

            let released_key = {
                let keyboard = self.keyboard.lock().unwrap();
                if let Some(key) = pressed_key {
                    if !keyboard.is_pressed(key as usize) {
                        Some(key)
                    } else {
                        None
                    }
                } else {
                    pressed_key = keyboard.is_key_pressed();
                    None
                }
            };

            if let Some(key) = released_key {
                self.cpu.set_general_register(register, key);
                return;
            }
            thread::sleep(Duration::from_millis(1));
        }
    }

    /// Executes an `FxNN` instruction.
    fn execute_f_instruction(&mut self, x: u16, code: u16) {
        match code {
            0x07 => {
                self.cpu.set_general_register(x, self.timers.get_delay());
            }
            0x15 => {
                self.timers.set_delay(self.cpu.get_general_register(x));
            }
            0x18 => {
                self.timers.set_sound(self.cpu.get_general_register(x));
            }
            0x1E => {
                let i_value = self.cpu.get_i_register();
                let vx_value = self.cpu.get_general_register(x);

                let (result, carry) = i_value.overflowing_add(vx_value as u16);

                self.cpu.set_i_register(result);
                self.cpu.set_general_register(0xF, carry as u8);
            }
            0x29 => {
                let vx_value = self.cpu.get_general_register(x);

                self.cpu
                    .set_i_register(ram::FONT_START_ADDRESS + (vx_value as u16 * 5));
            }
            0x33 => {
                let i_value = self.cpu.get_i_register();
                let vx_value = self.cpu.get_general_register(x);

                self.ram.write(i_value, vx_value / 100);
                self.ram.write(i_value + 1, (vx_value / 10) % 10);
                self.ram.write(i_value + 2, vx_value % 10);
            }
            0x55 => {
                let i_value = self.cpu.get_i_register();

                for i in 0..=x {
                    self.ram
                        .write(i_value + i, self.cpu.get_general_register(i));
                }
            }
            0x65 => {
                let i_value = self.cpu.get_i_register();

                for i in 0..=x {
                    let value = self.ram.read(i_value + i);

                    self.cpu.set_general_register(i, value);
                }
            }
            0x0A => self.execute_blocking_get_key(x),
            _ => println!("unsupported instruction: {:#06X}", 0xF000 | (x << 8) | code),
        }
    }

    /// Reads the next big-endian instruction and advances the program counter.
    fn fetch_instruction(&mut self) -> u16 {
        let opcode = u16::from_be_bytes([
            self.ram.read(self.cpu.get_pc()),
            self.ram.read(self.cpu.get_pc() + 1),
        ]);

        self.cpu.increment_pc();

        opcode
    }

    /// Splits an instruction into its four nibbles.
    fn decode_instruction(&self, instruction: u16) -> [u16; 4] {
        let n1 = (instruction >> 12) & 0xF;
        let n2 = (instruction >> 8) & 0xF;
        let n3 = (instruction >> 4) & 0xF;
        let n4 = instruction & 0xF;

        [n1, n2, n3, n4]
    }
}
