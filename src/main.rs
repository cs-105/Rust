mod input;
mod music;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Rect, Point};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use std::thread;

//imports from local crate
use input::controller::Input::*;
use input::controller::Input;
use input::controller::Player;
use input::controller::create_player;
use input::controller::remove_input;

//imports from music file
use crate::music::music::music;
use crate::music::music::music2;

//defining constants
//dimensions and title of the window to be rendered
const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const WINDOW_TITLE: &str = "The Game";

//dimensions of the player sprite
const PLAYER_SPRITE_WIDTH: u32 = 150;
const PLAYER_SPRITE_HEIGHT: u32 = 150;

const PLAYER_MOVEMENT_SPEED: u32 = 5;
const PLAYER_ROTATION_SPEED: u32 = 5;

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &mut Player,
) -> Result<(), String> {

    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = player.get_position() + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.get_sprite().width(), player.get_sprite().height());

    canvas.copy_ex(texture, player.get_sprite(), screen_rect, player.get_heading(), None, false, false)?;

    canvas.present();
    
    Ok(())

}

//TODO: Go through and outsource certain things to different files
fn main() -> Result<(), String> {
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build().unwrap();

    let mut canvas : WindowCanvas = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    //create the player ship sprite from an image
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/ship.png")?;

    let mut player = create_player(Point::new(0,0), Rect::new(0,0,PLAYER_SPRITE_WIDTH,PLAYER_SPRITE_HEIGHT), PLAYER_MOVEMENT_SPEED, PLAYER_ROTATION_SPEED, -90.0);

    let mut input_stack: Vec<Input> = Vec::with_capacity(6);

    let mut event_pump = sdl_context.event_pump()?;



    music2();
    //game loop
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
                    
                    input_stack.push(Up);

                },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    
                    input_stack.push(Left);

                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    
                    input_stack.push(Down)

                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    
                    input_stack.push(Right);

                },
                Event::KeyDown { keycode: Some(Keycode::E), repeat: false, .. } => {
                    
                    input_stack.push(RotateRight);

                },
                Event::KeyDown { keycode: Some(Keycode::Q), repeat: false, .. } => {
                    
                    input_stack.push(RotateLeft);

                },
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {

                    remove_input(&mut input_stack, &Up);
                    
                },
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
                    
                    remove_input(&mut input_stack, &Left);

                },
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                    
                    remove_input(&mut input_stack, &Down);

                },
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
                    
                    remove_input(&mut input_stack, &Right);

                },Event::KeyUp { keycode: Some(Keycode::E), repeat: false, .. } => {
                    
                    remove_input(&mut input_stack, &RotateRight);

                },
                Event::KeyUp { keycode: Some(Keycode::Q), repeat: false, .. } => {
                    
                    remove_input(&mut input_stack, &RotateLeft);

                },
                _ => {}

            }

        }

    //Update
    player.update_player(&mut input_stack);

    //draw to screen
    render(&mut canvas, Color::RGB(0,0,0), &texture, &mut player)?;

    //Lmimt to 144 fps
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }

    Ok(())

}