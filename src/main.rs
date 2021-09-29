extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Rect, Point};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use std::thread;

const SCREEN_WIDTH: u32 = 2560;
const SCREEN_HEIGHT: u32 = 1440;

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    position: Point,
    sprite: Rect,
) -> Result<(), String> {

    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_rect)?;

    canvas.present();

    Ok(())

}

fn main() -> Result<(), String> {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("sdl2", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build().unwrap();

    let mut canvas : WindowCanvas = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let position = Point::new(-100,200);
    let sprite = Rect::new(0,0,26,36);

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop{

        for event in event_pump.poll_iter(){

            match event{

                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}

            }

        }

    i = (i + 1) % 255;

    render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, position, sprite)?;

    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }

    Ok(())

}