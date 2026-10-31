use std::path::{Path, PathBuf};
use std::process::exit;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use crate::interpreter::Interpreter;

mod ram;
pub mod fonts;
pub mod stack;
pub mod timers;
pub mod input;
pub mod cpu;
pub mod display;
pub mod interpreter;

fn main() {
    /*let sdl_context = sdl3::init().expect("error while getting sdl context");
    let video = sdl_context.video().expect("error while getting sdl video");

    let window = video
        .window("Pixie", 640, 320)
        .position_centered()
        .build()
        .expect("error while building window");

    let mut canvas = window.into_canvas();


    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.present();
    }*/

    let mut interpreter = Interpreter::new();

    let program_path = Path::new("programs/IBM Logo.ch8");

    interpreter.load_program(program_path.to_path_buf()).expect("error while loading program");

    interpreter.start_execution();
}
