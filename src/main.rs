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

//XAxis Enum tracks the states of the x axis inputs given by the AD keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XAxis {

    Left,
    Right,
    Off,
    Both,

}

//YAxis Enum tracks the states of the y axis inputs given by the WS keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YAxis {

    Up,
    Down,
    Off,
    Both,

}

//Player Struct keeps track of data about the player avatar
#[derive(Debug)]
struct Player{

    position: Point, //2-D Cartesian Point 
    sprite: Rect, //Dimensions are used to select what to render from the spritesheet
    speed: i32, //Rate at which position is updated
    direction: (XAxis, YAxis), //Keeps track of what buttons are being pressed for processing with the update_player function

}

//Function to update the player's position on the screen, processes the direction component of the player passed in
//TODO: Update function so it moves by player speed on the diagonals too
fn update_player(player: &mut Player){

    use self::XAxis::*;
    use self::YAxis::*;

    match player.direction.0{
        Left =>{
            player.position = player.position.offset(-player.speed, 0);
        },
        Right =>{
            player.position = player.position.offset(player.speed, 0);},
        XAxis::Off | XAxis::Both =>{},
    };
    match player.direction.1{
        Up =>{
            player.position = player.position.offset(0, -player.speed);},
        Down =>{
            player.position = player.position.offset(0, player.speed);},
        YAxis::Off | YAxis::Both =>{},
    };

}

//update the canvas that is passed in, handles drawing the player sprite into the new position
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

    //create the player ship sprite
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/ship.png")?;

    let mut player = Player{

        position: Point::new(0,0),
        sprite: Rect::new(0,0,150,150),
        speed: 5,
        direction: (XAxis::Off, YAxis::Off),

    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop{

        for event in event_pump.poll_iter(){
            //handling input
            match event{
                //Quit logic
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                //Update direction enums when keys are pressed and released
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
                    if(player.direction.1 == YAxis::Down){
                        player.direction.1 = YAxis::Both;
                    } else {
                        player.direction.1 = YAxis::Up;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    if(player.direction.0 == XAxis::Right){
                        player.direction.0 = XAxis::Both;
                    } else {
                        player.direction.0 = XAxis::Left;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    if(player.direction.1 == YAxis::Up){
                        player.direction.1 = YAxis::Both;
                    } else {
                        player.direction.1 = YAxis::Down;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    if(player.direction.0 == XAxis::Left){
                        player.direction.0 = XAxis::Both;
                    } else {
                        player.direction.0 = XAxis::Right;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {
                    if(player.direction.1 == YAxis::Both){
                        player.direction.1 = YAxis::Down;
                    } else {
                        player.direction.1 = YAxis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
                    if(player.direction.0 == XAxis::Both){
                        player.direction.0 = XAxis::Right;
                    } else {
                        player.direction.0 = XAxis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                    if(player.direction.1 == YAxis::Both){
                        player.direction.1 = YAxis::Up;
                    } else {
                        player.direction.1 = YAxis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
                    if(player.direction.0 == XAxis::Both){
                        player.direction.0 = XAxis::Left;
                    } else {
                        player.direction.0 = XAxis::Off;
                    }
                },
                _ => {}

            }

        }

    //Update
    i = (i + 1) % 255;
    update_player(&mut player);

    render(&mut canvas, Color::RGB(0,0,0), &texture, &player)?;

    //Lmimt to 60 fps
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }

    Ok(())

}