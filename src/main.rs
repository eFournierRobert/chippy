use std::process::exit;
use sdl3::pixels::Color;

mod ram;
pub mod fonts;
pub mod stack;
pub mod timers;
pub mod input;
pub mod cpu;
pub mod display;

fn main() {
    let sdl_context = match sdl3::init() {
        Ok(sdl) => sdl,
        Err(e) => {
            eprintln!("error while getting sdl context: {}", e);
            exit(-1);
        }
    };
    let video = match sdl_context.video() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error while getting sdl video: {}", e);
            exit(-1);
        }
    };

    let window = match video
        .window("Pixie", 640, 320)
        .position_centered()
        .build() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("error while building window: {}", e);
            exit(-1);
        }
    };

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();

    loop{}
}
