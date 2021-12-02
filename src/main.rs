mod asteroid;
mod game_object;
mod input;
mod music;
mod player;

extern crate sdl2;
use crate::asteroid::asteroid::Asteroid;
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
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::GLProfile;
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
    if (num.abs() > 0.2) {
        return num;
    } else {
        return 0.0;
    }
}

//TODO: Go through and outsource certain things to different files
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    sdl2::hint::set_with_priority(
        "SDL_HINT_RENDER_SCALE_QUALITY",
        "2",
        &sdl2::hint::Hint::Override,
    );

    let gl_attr = video_subsystem.gl_attr();

    // Don't use deprecated OpenGL functions
    gl_attr.set_context_profile(GLProfile::Core);
    // Enable anti-aliasing
    gl_attr.set_multisample_samples(4);

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    let mut player = Player {
        texture: texture_creator.load_texture("assets/ship.png")?,
        position: Rect::new(500, 500, 150, 150),
        pos: Vec2::new(
            SCREEN_WIDTH as f32 / 2.0 - 150.0 / 2.0,
            SCREEN_HEIGHT as f32 / 2.0 - 150.0 / 2.0,
        ),
        angle: 0.0,
        velocity: Vec2::new(0.0, 0.0),
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
    // Starting the main menu soundtrack
    let music_thread = thread::spawn(|| main_menu_music());

    let mut asteroids: Vec<Asteroid> = Vec::new();
    asteroids.push(Asteroid::new_with_position(
        texture_creator.load_texture("assets/asteroid.png")?,
        Vec2::new(50.0, 50.0),
    ));

    asteroids.push(Asteroid::new_with_position(
        texture_creator.load_texture("assets/asteroid.png")?,
        Vec2::new(150.0, 150.0),
    ));

    asteroids.push(Asteroid::new_with_position(
        texture_creator.load_texture("assets/asteroid.png")?,
        Vec2::new(500.0, 500.0),
    ));

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

        let k_input = KeyboardInput {
            forward: keys.get(&Keycode::W).is_some(),
            back: keys.get(&Keycode::S).is_some(),
            left: keys.get(&Keycode::A).is_some(),
            right: keys.get(&Keycode::D).is_some(),

            rotate_left: keys.get(&Keycode::Q).is_some(),
            rotate_right: keys.get(&Keycode::E).is_some(),
        };

        let c_input: ControllerInput;

        if let Some(c) = &controller {
            c_input = ControllerInput {
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
            c_input = ControllerInput {
                left: (0.0, 0.0),
                right: (0.0, 0.0),
            };
        }

        let delta_duration = now.elapsed() - old_time;
        let delta_seconds = delta_duration.as_millis() as f64;

        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        player.update(delta_seconds / 100.0, k_input, c_input);

        for asteroid in asteroids.iter_mut() {
            asteroid.update(delta_seconds / 100.0, k_input, c_input);
            asteroid.render(&mut canvas);
        }
        player.render(&mut canvas);

        canvas.present();

        old_time = now.elapsed();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}
