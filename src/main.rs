extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Rect, Point};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use std::thread;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum X_Axis {

    Left,
    Right,
    Off,
    Both,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Y_Axis {

    Up,
    Down,
    Off,
    Both,

}

#[derive(Debug)]
struct Player{

    position: Point,
    sprite: Rect,
    speed: i32,
    direction: (X_Axis, Y_Axis),

}

fn update_player(player: &mut Player){

    use self::X_Axis::*;
    use self::Y_Axis::*;

    match player.direction.0{
        Left =>{
            player.position = player.position.offset(-player.speed, 0);
        },
        Right =>{
            player.position = player.position.offset(player.speed, 0);},
        X_Axis::Off | X_Axis::Both =>{},
    };
    match player.direction.1{
        Up =>{
            player.position = player.position.offset(0, -player.speed);},
        Down =>{
            player.position = player.position.offset(0, player.speed);},
        Y_Axis::Off | Y_Axis::Both =>{},
    };

}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {

    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    canvas.copy(texture, player.sprite, screen_rect)?;

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
    let texture = texture_creator.load_texture("assets/ship.png")?;

    let mut player = Player{

        position: Point::new(0,0),
        sprite: Rect::new(0,0,150,150),
        speed: 5,
        direction: (X_Axis::Off, Y_Axis::Off),

    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop{

        for event in event_pump.poll_iter(){

            match event{

                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                }, 
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
                    if(player.direction.1 == Y_Axis::Down){
                        player.direction.1 = Y_Axis::Both;
                    } else {
                        player.direction.1 = Y_Axis::Up;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    if(player.direction.0 == X_Axis::Right){
                        player.direction.0 = X_Axis::Both;
                    } else {
                        player.direction.0 = X_Axis::Left;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    if(player.direction.1 == Y_Axis::Up){
                        player.direction.1 = Y_Axis::Both;
                    } else {
                        player.direction.1 = Y_Axis::Down;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    if(player.direction.0 == X_Axis::Left){
                        player.direction.0 = X_Axis::Both;
                    } else {
                        player.direction.0 = X_Axis::Right;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {
                    if(player.direction.1 == Y_Axis::Both){
                        player.direction.1 = Y_Axis::Down;
                    } else {
                        player.direction.1 = Y_Axis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
                    if(player.direction.0 == X_Axis::Both){
                        player.direction.0 = X_Axis::Right;
                    } else {
                        player.direction.0 = X_Axis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                    if(player.direction.1 == Y_Axis::Both){
                        player.direction.1 = Y_Axis::Up;
                    } else {
                        player.direction.1 = Y_Axis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
                    if(player.direction.0 == X_Axis::Both){
                        player.direction.0 = X_Axis::Left;
                    } else {
                        player.direction.0 = X_Axis::Off;
                    }
                },
                _ => {}

            }

        }

    //Update
    i = (i + 1) % 255;
    update_player(&mut player);

    render(&mut canvas, Color::RGB(0,0,0), &texture, &player)?;

    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }

    Ok(())

}