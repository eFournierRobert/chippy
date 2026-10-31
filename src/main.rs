use std::process::exit;
use sdl3::pixels::Color;
use sdl3::rect::Rect;

mod ram;
pub mod fonts;
pub mod stack;
pub mod timers;
pub mod input;
pub mod cpu;
pub mod display;

fn main() {
    let sdl_context = sdl3::init().expect("error while getting sdl context");
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
    }
}
