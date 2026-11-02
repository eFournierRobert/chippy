use crate::interpreter::Interpreter;
use std::path::Path;

pub mod cpu;
pub mod display;
pub mod fonts;
pub mod input;
pub mod interpreter;
mod ram;
pub mod stack;
pub mod timers;

fn main() {
    let mut interpreter = Interpreter::new();

    let program_path = Path::new("programs/Keypad Test [Hap, 2006].ch8");

    interpreter
        .load_program(program_path.to_path_buf())
        .expect("error while loading program");

    interpreter.start_execution();
}
