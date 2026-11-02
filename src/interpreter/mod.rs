use crate::cpu::Cpu;
use crate::display::Display;
use crate::input::Keyboard;
use crate::ram::Ram;
use crate::stack::Stack;
use crate::timers::Timers;
use crate::{display, fonts, ram};
use std::fs::File;
use std::io::{stdout, Error, Read};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const CPU_FREQUENCY: u64 = 500;
const CYCLE_DURATION: Duration = Duration::from_nanos(1_000_000_000 / CPU_FREQUENCY);

pub struct Interpreter {
    cpu: Cpu,
    display: Display,
    keyboard: Arc<Mutex<Keyboard>>,
    ram: Ram,
    stack: Stack,
    timers: Timers,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            cpu: Cpu::new(),
            display: Display::new(),
            keyboard: Arc::new(Mutex::new(Keyboard::new())),
            ram: Ram::new(),
            stack: Stack::new(),
            timers: Timers::new(),
        };

        for (i, byte) in fonts::FONT.iter().flatten().enumerate() {
            interpreter
                .ram
                .write(ram::FONT_START_ADDRESS + i as u16, *byte);
        }

        interpreter
    }

    pub fn load_program(&mut self, path: PathBuf) -> Result<(), Error> {
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
        enable_raw_mode().expect("can't enable raw mode");
        execute!(
            stdout(),
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
            )
        ).expect("can't enable stdout enhancement flags");

        let input_keyboard = Arc::clone(&self.keyboard);
        let quit = Arc::new(AtomicBool::new(false));
        let input_quit = Arc::clone(&quit);

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

        let mut instruction = self.fetch_instruction();
        'execution: while instruction != 0 && !quit.load(Ordering::Relaxed) {
            let start = Instant::now();
            self.timers.tick();
            let decoded_instruction = self.decode_instruction(instruction);

            match decoded_instruction[0] {
                0x0 => {
                    if decoded_instruction[1] == 0
                        && decoded_instruction[2] == 0xE
                        && decoded_instruction[3] == 0
                    {
                        self.display.clear_display();
                        self.display.print_display();
                    }

                    if decoded_instruction[1] == 0
                        && decoded_instruction[2] == 0xE
                        && decoded_instruction[3] == 0xE
                    {
                        let new_pc = self.stack.pop().expect("cannot pop empty stack");
                        self.cpu.set_pc(new_pc);
                    }
                }
                0x1 => {
                    self.cpu.set_pc(instruction & 0x0FFF);
                }
                0x6 => {
                    self.cpu
                        .set_general_register(decoded_instruction[1], (instruction & 0x00FF) as u8);
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
                0x2 => {
                    self.stack.push(self.cpu.get_pc());
                    self.cpu.set_pc(instruction & 0x0FFF);
                }
                0x3 => {
                    let addr = decoded_instruction[1];
                    let value = instruction & 0x00FF;

                    if self.cpu.get_general_register(addr) == value as u8 {
                        self.cpu.increment_pc();
                    }
                }
                0x4 => {
                    let addr = decoded_instruction[1];
                    let value = instruction & 0x00FF;

                    if self.cpu.get_general_register(addr) != value as u8 {
                        self.cpu.increment_pc();
                    }
                }
                0x5 => {
                    let first_addr = decoded_instruction[1];
                    let second_addr = decoded_instruction[2];

                    if self.cpu.get_general_register(first_addr) == self.cpu.get_general_register(second_addr) {
                        self.cpu.increment_pc();
                    }
                }
                0x9 => {
                    let first_addr = decoded_instruction[1];
                    let second_addr = decoded_instruction[2];

                    if self.cpu.get_general_register(first_addr) != self.cpu.get_general_register(second_addr) {
                        self.cpu.increment_pc();
                    }
                }
                0x8 => {
                    let vx = decoded_instruction[1];
                    let vy = decoded_instruction[2];

                    let vx_value = self.cpu.get_general_register(vx);
                    let vy_value = self.cpu.get_general_register(vy);

                    match decoded_instruction[3] {
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
                            self.cpu.add_registers(
                                vx,
                                vy
                            );
                        }
                        0x5 => {
                            self.cpu.subtract_value_to_general_register(
                                vx,
                                vy_value,
                                None
                            )
                        }
                        0x7 => {
                            self.cpu.subtract_value_to_general_register(
                                vy,
                                vx_value,
                                Some(vx)
                            )
                        }
                        0x6 => {
                            self.cpu.shift_right(vx)
                        }
                        0xE => {
                            self.cpu.shift_left(vx)
                        }
                        _ => println!("unsupported instruction: {:?}", decoded_instruction)
                    }
                }
                0xB => {
                    let v0_value = self.cpu.get_general_register(0x0);
                    self.cpu.set_pc(v0_value as u16 + (instruction & 0x0FFF));
                }
                0xC => {
                    let mut value = rand::random_range(0..254);
                    value &= (instruction & 0x00FF);
                    self.cpu.set_general_register(decoded_instruction[1], value as u8);
                }
                0xF => {
                    let code = instruction & 0x00FF;

                    match code {
                        0x07 => {
                            self.cpu.set_general_register(
                                decoded_instruction[1],
                                self.timers.get_delay()
                            );
                        }
                        0x15 => {
                            self.timers.set_delay(
                                self.cpu.get_general_register(decoded_instruction[1])
                            );
                        }
                        0x18 => {
                            self.timers.set_sound(
                                self.cpu.get_general_register(decoded_instruction[1])
                            );
                        }
                        0x1E => {
                            let i_value = self.cpu.get_i_register();
                            let vx_value = self.cpu.get_general_register(decoded_instruction[1]);

                            let (result, carry) = i_value.overflowing_add(vx_value as u16);

                            self.cpu.set_i_register(result);
                            self.cpu.set_general_register(0xF, carry as u8);
                        }
                        0x29 => {
                            let vx_value = self.cpu.get_general_register(decoded_instruction[1]);

                            self.cpu.set_i_register(ram::FONT_START_ADDRESS + (vx_value as u16 * 5));
                        }
                        0x33 => {
                            let i_value = self.cpu.get_i_register();
                            let vx_value = self.cpu.get_general_register(decoded_instruction[1]);

                            self.ram.write(i_value, vx_value / 100);
                            self.ram.write(i_value + 1, (vx_value / 10) % 10);
                            self.ram.write(i_value + 2, vx_value % 10);
                        }
                        0x55 => {
                            let i_value = self.cpu.get_i_register();

                            for i in 0..=decoded_instruction[1] {
                                self.ram.write(
                                    i_value + i,
                                    self.cpu.get_general_register(i)
                                );
                            }
                        }
                        0x65 => {
                            let i_value = self.cpu.get_i_register();

                            for i in 0..=decoded_instruction[1] {
                                let value = self.ram.read(i_value + i);

                                self.cpu.set_general_register(i, value);
                            }
                        }
                        0x0A => {
                            let mut key_pressed = false;

                            while !key_pressed {
                                if quit.load(Ordering::Relaxed) {
                                    break 'execution;
                                }
                                self.timers.tick();

                                if let Some(v) = self.keyboard.lock().unwrap().is_key_pressed() {
                                    key_pressed = true;
                                    self.cpu.set_general_register(decoded_instruction[1], v);
                                }
                            }
                        }
                        _ => println!("unsupported instruction: {:?}", decoded_instruction)
                    }
                }
                0xE => {
                    let code = instruction & 0x00FF;
                    let vx_value = self.cpu.get_general_register(decoded_instruction[1]);

                    match code {
                        0x9E => {
                            if self.keyboard.lock().unwrap().is_pressed(vx_value as usize) {
                                self.cpu.increment_pc();
                            }
                        }
                        0xA1 => {
                            if !self.keyboard.lock().unwrap().is_pressed(vx_value as usize) {
                                self.cpu.increment_pc();
                            }
                        }
                        _ => println!("unsupported instruction: {:?}", decoded_instruction)
                    }
                }
                _ => {
                    println!("unsupported instruction: {:?}", decoded_instruction);
                }
            }

            instruction = self.fetch_instruction();

            let elapsed = start.elapsed();
            if elapsed < CYCLE_DURATION {
                thread::sleep(CYCLE_DURATION - elapsed);
            }
        }

        disable_raw_mode().expect("can't disable raw mode");
        execute!(
            stdout(),
            PopKeyboardEnhancementFlags
        ).expect("can't pop keyboard enhancement flags");
    }

    fn fetch_instruction(&mut self) -> u16 {
        let opcode = u16::from_be_bytes([
            self.ram.read(self.cpu.get_pc()),
            self.ram.read(self.cpu.get_pc() + 1),
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
