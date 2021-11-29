mod game_object;
mod input;
mod music;
mod player;

extern crate sdl2;
use crate::game_object::game_object::ControllerInput;
use crate::game_object::game_object::GameObject;
use crate::game_object::game_object::KeyboardInput;
use crate::game_object::game_object::Renderable;
use crate::player::player::Player;
use glam::Vec2;
use sdl2::controller::Axis;
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::time::Instant;

//imports from music file
use crate::music::music::in_game_music;
use crate::music::music::main_menu_music;

//defining constants
//dimensions and title of the window to be rendered
const SCREEN_WIDTH: u32 = 1280; //Width in pixels
const SCREEN_HEIGHT: u32 = 720; //Height in pixels
const WINDOW_TITLE: &str = "The Game";

fn clamp(num: f32) -> f32 {
    if (num.abs() > 0.1) {
        return num;
    } else {
        return 0.0;
    }
}

//TODO: Go through and outsource certain things to different files
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.clear();
    canvas.present();

    //create the player ship sprite from an image
    // let texture = texture_creator.load_texture("assets/ship.png")?;

    // let mut player = create_player(
    //     Point::new(0, 0),
    //     Rect::new(0, 0, PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
    //     PLAYER_MOVEMENT_SPEED,
    //     texture,
    //     PLAYER_ROTATION_SPEED,
    //     -90.0,
    // );
    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    let mut player = Player {
        texture: texture_creator.load_texture("assets/ship.png")?,
        position: Rect::new(500, 500, 150, 150),
        pos: Vec2::new(2.0, 2.0),
        angle: 0.0,
    };

    let game_controller_subsystem = sdl_context.game_controller()?;

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);

    let mut controller: Option<GameController> = (0..available).find_map(|id| {
        println!("con");
        if !game_controller_subsystem.is_game_controller(id) {
            println!("{} is not a game controller", id);
            return None;
        }

        println!("Attempting to open controller {}", id);

        match game_controller_subsystem.open(id) {
            Ok(c) => {
                // We managed to find and open a game controller,
                // exit the loop
                println!("Success: opened \"{}\"", c.name());
                Some(c)
            }
            Err(e) => {
                println!("failed: {:?}", e);
                None
            }
        }
    });

    let now = Instant::now();
    let mut old_time: Duration = now.elapsed();
    let music_thread = thread::spawn(|| main_menu_music());
    //game loop
    'running: loop {
        //handling input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let keys: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let kInput = KeyboardInput {
            forward: keys.get(&Keycode::W).is_some(),
            back: keys.get(&Keycode::S).is_some(),
            left: keys.get(&Keycode::A).is_some(),
            right: keys.get(&Keycode::D).is_some(),

            rotate_left: keys.get(&Keycode::Q).is_some(),
            rotate_right: keys.get(&Keycode::E).is_some(),
        };

        let cInput: ControllerInput;

        if let Some(c) = &controller {
            cInput = ControllerInput {
                left: (
                    clamp(c.axis(Axis::LeftX) as f32 / i16::MAX as f32),
                    clamp(c.axis(Axis::LeftY) as f32 / i16::MAX as f32),
                ),
                right: (
                    clamp(c.axis(Axis::RightX) as f32 / i16::MAX as f32),
                    clamp(c.axis(Axis::RightY) as f32 / i16::MAX as f32),
                ),
            };
        } else {
            cInput = ControllerInput {
                left: (0.0, 0.0),
                right: (0.0, 0.0),
            };
        }

        let delta_duration = now.elapsed() - old_time;
        let delta_seconds = delta_duration.as_millis() as f64;

        player.update(delta_seconds / 100.0, kInput, cInput);
        player.render(&mut canvas);
        old_time = now.elapsed();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}
