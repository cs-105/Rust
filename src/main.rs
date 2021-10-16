mod input;

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
use input::controller::XAxis;
use input::controller::YAxis;
use input::controller::Input::*;
use input::controller::Input;
use input::controller::Vector;
use input::controller::Player;
use input::controller::transform_vector;
use input::controller::remove_input;

//defining constants
//dimensions and title of the window to be rendered
const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const WINDOW_TITLE: &str = "The Game";

//dimensions of the player sprite
const PLAYER_SPRITE_WIDTH: u32 = 150;
const PLAYER_SPRITE_HEIGHT: u32 = 150;

const PLAYER_MOVEMENT_SPEED: u32 = 10;

//TODO: Move to separate file
    //Function to update the player's position on the screen, processes the direction component of the player passed in
    //TODO: Update function so it moves by player velocity on the diagonals too
    pub fn update_player(player: &mut Player, inputs: &mut Vec<Input>){

        // use self::XAxis::*;
        // use self::YAxis::*;

        use self::Input::*;

        //velocity vectors relative to the player's heading
        let mut velocity_x = Vector{

            magnitude: 0.0,
            direction: 0.0,

        };
        let mut velocity_y = Vector{

            magnitude: 0.0,
            direction: 0.0,

        };

        for input in inputs.iter(){

            match input{

                Up => {

                    velocity_y.magnitude += player.speed as f64;
                    velocity_y.direction = 0.0;

                },
                Down => {

                    velocity_y.magnitude -= player.speed as f64;
                    velocity_y.direction = 0.0;

                },
                Left => {

                    velocity_x.magnitude += player.speed as f64;
                    velocity_x.direction = 90.0;

                },
                Right => {

                    velocity_x.magnitude -= player.speed as f64;
                    velocity_x.direction = 90.0;

                }

            };

        }

        let (offset_x, offset_y) = transform_vector(velocity_x, velocity_y, player.heading);

        player.position = player.position.offset(offset_x as i32, offset_y as i32);

        //check if the player is heading out of bounds on the x axis and undo the position change
        if (player.position.x - PLAYER_SPRITE_WIDTH as i32 / 2) < -(SCREEN_WIDTH as i32 / 2) || (player.position.x + PLAYER_SPRITE_WIDTH as i32 / 2) > SCREEN_WIDTH as i32 / 2{
            player.position = player.position.offset(-offset_x as i32, 0);
        }

        //check if the player is heading out of bounds on the y axis and undo the position change
        if (player.position.y - PLAYER_SPRITE_HEIGHT as i32 / 2) < -(SCREEN_HEIGHT as i32 / 2) || (player.position.y + PLAYER_SPRITE_HEIGHT as i32 /2) > SCREEN_HEIGHT as i32 / 2{
            player.position = player.position.offset(0,-offset_y as i32);
        }

    }

//TODO:Create function that procedurally generates a background

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

    let mut player = Player{

        position: Point::new(0,0),
        sprite: Rect::new(0,0,PLAYER_SPRITE_WIDTH,PLAYER_SPRITE_HEIGHT),
        speed: PLAYER_MOVEMENT_SPEED,
        direction: (XAxis::Off, YAxis::Off),
        heading: 0.0,

    };

    let mut input_stack: Vec<Input> = Vec::with_capacity(4);

    let mut event_pump = sdl_context.event_pump()?;

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

                },
                _ => {}

            }

        }

    //Update
    update_player(&mut player, &mut input_stack);

    //draw to screen
    render(&mut canvas, Color::RGB(0,0,0), &texture, &player)?;

    //Lmimt to 60 fps
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));

    }

    Ok(())

}