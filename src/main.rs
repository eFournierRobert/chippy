use crate::interpreter::Interpreter;
use clap::{Arg, command};
use std::path::Path;

/// CPU registers and arithmetic operations.
pub mod cpu;
/// CHIP-8 display buffer and terminal rendering.
pub mod display;
/// Built-in hexadecimal digit sprites.
pub mod fonts;
/// CHIP-8 keyboard state and terminal input.
pub mod input;
/// Program loading and instruction execution.
pub mod interpreter;
/// CHIP-8 memory.
pub mod ram;
/// Subroutine return address stack.
pub mod stack;
/// Delay and sound timers.
pub mod timers;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("game")
                .short('g')
                .long("game")
                .help("Game to launch")
                .required(true),
        )
        .arg_required_else_help(true)
        .about("A terminal Chip-8 interpreter")
        .author("Elliott Fournier-Robert")
        .get_matches();

    if let Some(game) = matches.get_one::<String>("game") {
        print!("\x1B[2J");
        let mut interpreter = Interpreter::default();

        let program_path = Path::new(game);

        interpreter
            .load_program(program_path.to_path_buf())
            .expect("error while loading program");

        interpreter.start_execution();
    }
}
