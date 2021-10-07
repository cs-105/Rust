mod util::test::Test;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Rect, Point};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use std::thread;

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
//XAxis Enum tracks the states of the x axis inputs given by the AD keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XAxis {

    Left,
    Right,
    Off,
    Both,

}

//TODO: Move to separate file
//YAxis Enum tracks the states of the y axis inputs given by the WS keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YAxis {

    Up,
    Down,
    Off,
    Both,

}

//TODO: Move to separate file
//Physics vector, mainly used for storing velocity
#[derive(Debug, Clone, Copy)]
struct Vector{

    magnitude: f64,
    direction: f64, 

}

//TODO: Move to separate file
//Player Struct keeps track of data about the player avatar
#[derive(Debug)]
struct Player{

    position: Point, //2-D Cartesian Point 
    sprite: Rect, //Dimensions are used to select what to render from the spritesheet
    speed: u32, //Rate at which the player sprite is moved
    direction: (XAxis, YAxis), //Keeps track of what buttons are being pressed for processing with the update_player function
    heading: f64, //Heading of the player

}

//TODO: Move to separate file
//gets x and y components of a vector
fn get_components(vector: Vector) -> (f64, f64){

    let result_x = vector.magnitude * vector.direction.to_radians().cos();
    let result_y = vector.magnitude * vector.direction.to_radians().sin();

    (result_x, result_y)

}

//TODO: Move to separate file
//Transforms the x and y velocty vectors into coordinates to offset
fn transform_vector(velocity_x: Vector, velocity_y: Vector, heading: f64) -> (f64, f64){

    //vectors transformed to match the unit circle
    let transformed_x = Vector{

        magnitude: velocity_x.magnitude,
        direction: heading + 90.0 + velocity_x.direction,

    };
    let transformed_y = Vector{

        magnitude: velocity_y.magnitude,
        direction: heading + 90.0 + velocity_y.direction,

    };

    let offset_x = get_components(transformed_x).0 + get_components(transformed_y).0;
    let offset_y = get_components(transformed_x).1 + get_components(transformed_y).1;

    (offset_x, -offset_y)

}

//TODO: Move to separate file
//Function to update the player's position on the screen, processes the direction component of the player passed in
//TODO: Update function so it moves by player velocity on the diagonals too
fn update_player(player: &mut Player){

    use self::XAxis::*;
    use self::YAxis::*;

    //velocity vectors relative to the player's heading
    let mut velocity_x = Vector{

        magnitude: 0.0,
        direction: 0.0,

    };
    let mut velocity_y = Vector{

        magnitude: 0.0,
        direction: 0.0,

    };

    match player.direction.0{

        Left => {

            velocity_x.magnitude = player.speed as f64;
            velocity_x.direction = 90.0;

        },
        Right => {

            velocity_x.magnitude = player.speed as f64;
            velocity_x.direction = 270.0;

        },
        XAxis::Off |
        XAxis::Both => {

            velocity_x.magnitude = 0.0;
            velocity_x.direction = 0.0;

        },

    };
    match player.direction.1 {
        Up => {

            velocity_y.magnitude = player.speed as f64;
            velocity_y.direction = 0.0;

        },
        Down => {

            velocity_y.magnitude = player.speed as f64;
            velocity_y.direction = 180.0;

        },
        YAxis::Off |
        YAxis::Both => {

            velocity_y.magnitude = 0.0;
            velocity_y.direction = 0.0;

        },
    };

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
                    if player.direction.1 == YAxis::Down {
                        player.direction.1 = YAxis::Both;
                    } else {
                        player.direction.1 = YAxis::Up;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    if player.direction.0 == XAxis::Right {
                        player.direction.0 = XAxis::Both;
                    } else {
                        player.direction.0 = XAxis::Left;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    if player.direction.1 == YAxis::Up {
                        player.direction.1 = YAxis::Both;
                    } else {
                        player.direction.1 = YAxis::Down;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    if player.direction.0 == XAxis::Left {
                        player.direction.0 = XAxis::Both;
                    } else {
                        player.direction.0 = XAxis::Right;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {
                    if player.direction.1 == YAxis::Both {
                        player.direction.1 = YAxis::Down;
                    } else {
                        player.direction.1 = YAxis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
                    if player.direction.0 == XAxis::Both {
                        player.direction.0 = XAxis::Right;
                    } else {
                        player.direction.0 = XAxis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                    if player.direction.1 == YAxis::Both {
                        player.direction.1 = YAxis::Up;
                    } else {
                        player.direction.1 = YAxis::Off;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
                    if player.direction.0 == XAxis::Both {
                        player.direction.0 = XAxis::Left;
                    } else {
                        player.direction.0 = XAxis::Off;
                    }//FIXME: This can be implemented better by storing inputs in a stack (VecDeque) and processing that in update_player
                },
                _ => {}

            }

        }

    //Update
    update_player(&mut player);

    //draw to screen
    render(&mut canvas, Color::RGB(0,0,0), &texture, &player)?;

    //Lmimt to 60 fps
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));

    }

    Ok(())

}