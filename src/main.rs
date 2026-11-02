use std::env::args;
use crate::interpreter::Interpreter;
use std::path::Path;
use clap::{command, Arg};

pub mod cpu;
pub mod display;
pub mod fonts;
pub mod input;
pub mod interpreter;
mod ram;
pub mod stack;
pub mod timers;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("game")
                .short('g')
                .long("game")
                .help("Game to launch")
                .required(true)
        )
        .arg_required_else_help(true)
        .about("A fully working terminal Chip-8 interpreter")
        .author("Elliott Fournier-Robert")
        .get_matches();

    if let Some(game) = matches.get_one::<String>("game") {
        print!("\x1B[2J");
        let mut interpreter = Interpreter::new();

        let program_path = Path::new(game);

        interpreter
            .load_program(program_path.to_path_buf())
            .expect("error while loading program");

        interpreter.start_execution();
    }
}
