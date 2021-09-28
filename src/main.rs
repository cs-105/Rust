extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::time::Duration;
use std::thread;

const SCREEN_WIDTH: u32 = 3200;
const SCREEN_HEIGHT: u32 = 1800;

fn main() {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem.window("sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
    //     .position_centered()
    //     .build()
    //     .unwrap();

    let window = video_subsystem.window("sdl2", SCREEN_WIDTH, SCREEN_HEIGHT).build().unwrap();

    let mut canvas : Canvas<Window> = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    canvas.set_draw_color(Color::RGB(255,255,255));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 210, 0));

    canvas.fill_rect(Rect::new(10, 10, 780, 580));

    canvas.present();

    thread::sleep(Duration::from_secs(5));


}
