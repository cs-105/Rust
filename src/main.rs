mod asteroid;
mod collide;
mod game;
mod game_object;
mod graphics;
mod input;
mod main_menu;
mod music;
mod player;

extern crate sdl2;
use crate::asteroid::asteroid::Asteroid;
use crate::asteroid::asteroid::AsteroidVariant;
use crate::collide::collide::is_colliding;
use crate::game::game::Game;
use crate::game_object::game_object::ControllerInput;
use crate::game_object::game_object::GameObject;
use crate::game_object::game_object::KeyboardInput;
use crate::game_object::game_object::Renderable;
use crate::graphics::graphics::render_text;
use crate::main_menu::main_menu::MainMenu;
use crate::player::player::set_vec_angle;
use crate::player::player::transform_to_ship_space;
use crate::player::player::Bullet;
use crate::player::player::Player;
use crate::player::player::PLAYER_SPRITE_HEIGHT;
use crate::player::player::PLAYER_SPRITE_WIDTH;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use glam::Vec2;
use sdl2::controller::Axis;
use sdl2::controller::Button;
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
use std::io;
use std::io::sink;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

//imports from music file
use crate::music::music::start_sound_thread;
use crate::music::music::{Sound, SoundType};

//defining constants
//dimensions and title of the window to be rendered
const WINDOW_TITLE: &str = "The Game";

fn clamp(num: f32) -> f32 {
    if (num.abs() > 0.2) {
        return num;
    } else {
        return 0.0;
    }
}

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
        .window(WINDOW_TITLE, 1920, 1080)
        .fullscreen_desktop()
        .opengl()
        .build()
        .unwrap();

    let mut window_size = window.size();

    let mut canvas: WindowCanvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    let mut player = Player {
        texture: texture_creator.load_texture("assets/ship.png")?,
        position: Rect::new(500, 500, 150, 150),
        pos: Vec2::new(
            window_size.0 as f32 / 2.0 - 150.0 / 2.0,
            window_size.1 as f32 / 2.0 - 150.0 / 2.0,
        ),
        angle: 0.0,
        velocity: Vec2::new(0.0, 0.0),
        previous_shoot: false,
    };

    let (tx, rx): (Sender<Sound>, Receiver<Sound>) = mpsc::channel();

    // Starting the main menu soundtrack
    let music_thread = thread::spawn(move || start_sound_thread(rx));
    let main_menu_sound = Sound::new("assets/Asteroids_MAIN_MENU.mp3", SoundType::Music);
    tx.send(main_menu_sound);

    let game_controller_subsystem = sdl_context.game_controller()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("assets/pressstart2p.ttf", 36)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);

    let mut controller: Option<GameController> = (0..available).find_map(|id| {
        if !game_controller_subsystem.is_game_controller(id) {
            println!("{} is not a game controller", id);
            return None;
        }

        println!("Attempting to open controller {}", id);

        match game_controller_subsystem.open(id) {
            Ok(c) => {
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

    let mut fps_instant = Instant::now();

    let mut asteroids: Vec<Asteroid> = Vec::new();

    for n in 1..20 {
        asteroids.push(Asteroid::new(
            texture_creator.load_texture("assets/asteroid.png")?,
        ));
    }

    let mut frames: f64 = 0.0;
    let mut frames_per_second: f64 = 0.0;

    let mut main_menu = MainMenu::new(window_size, &font, &texture_creator);
    let mut previous_main_menu = main_menu.continue_to_game;

    let mut game = Game::new(window_size, texture_creator, tx);

    //game loop
    'running: loop {
        frames = frames + 1.0;
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
            left: false,
            right: false,

            rotate_left: keys.get(&Keycode::A).is_some(),
            rotate_right: keys.get(&Keycode::D).is_some(),

            shoot: keys.get(&Keycode::Space).is_some(),
        };

        let c_input: ControllerInput;

        if let Some(c) = &controller {
            c_input = ControllerInput {
                left: (0.0, clamp(c.axis(Axis::LeftY) as f32 / i16::MAX as f32)),
                right: (
                    clamp(c.axis(Axis::RightX) as f32 / i16::MAX as f32),
                    clamp(c.axis(Axis::RightY) as f32 / i16::MAX as f32),
                ),
                shoot: c.button(Button::RightShoulder)
                    || clamp(c.axis(Axis::TriggerRight) as f32 / i16::MAX as f32) > 0.5,
            };
        } else {
            c_input = ControllerInput {
                left: (0.0, 0.0),
                right: (0.0, 0.0),
                shoot: false,
            };
        }

        let delta_duration = now.elapsed() - old_time;
        let delta_seconds = delta_duration.as_millis() as f64;

        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        if main_menu.continue_to_game && !previous_main_menu {
            game.start();
        }
        previous_main_menu = main_menu.continue_to_game;

        if !main_menu.continue_to_game {
            main_menu.update(window_size, delta_seconds / 100.0, k_input, c_input);
            main_menu.render(&mut canvas);
        } else {
            if !game.finished {
                game.update(window_size, delta_seconds / 100.0, k_input, c_input);
                game.render(&mut canvas);
                let score_texture = render_text(
                    format!("score: {}", game.score),
                    &font,
                    &game.texture_creator,
                )?;
                let score_texture_query = score_texture.query();
                let score_target = Rect::new(
                    (window_size.0 / 2) as i32 - (score_texture_query.width / 2) as i32,
                    0,
                    score_texture_query.width,
                    score_texture_query.height,
                );
                canvas.copy(&score_texture, None, score_target);

                let fps_texture = render_text(
                    format!("fps: {}", frames_per_second as i32),
                    &font,
                    &game.texture_creator,
                )
                .unwrap();
                let fps_texture_query = fps_texture.query();
                let fps_target = Rect::new(0, 0, fps_texture_query.width, fps_texture_query.height);
                canvas.copy(&fps_texture, None, fps_target);

                if game.dead {
                    let dead_texture =
                        render_text("DEAD".to_string(), &font, &game.texture_creator)?;
                    let dead_texture_query = dead_texture.query();
                    let dead_target = Rect::new(
                        (window_size.0 / 2) as i32 - (dead_texture_query.width / 2) as i32,
                        (window_size.1 / 2) as i32 - (dead_texture_query.height / 2) as i32,
                        dead_texture_query.width,
                        dead_texture_query.height,
                    );
                    canvas.copy(&dead_texture, None, dead_target);
                }
            } else {
                let thanks_texture = render_text(
                    "Thanks for playing!".to_string(),
                    &font,
                    &game.texture_creator,
                )?;
                let thanks_texture_query = thanks_texture.query();
                let thanks_target = Rect::new(
                    (window_size.0 / 2) as i32 - (thanks_texture_query.width / 2) as i32 - 25,
                    (window_size.1 / 2) as i32 - (thanks_texture_query.height / 2) as i32 - 25,
                    thanks_texture_query.width,
                    thanks_texture_query.height,
                );
                canvas.copy(&thanks_texture, None, thanks_target);

                let final_score_texture = render_text(
                    format!("Final Score: {}", game.score),
                    &font,
                    &game.texture_creator,
                )?;
                let final_score_texture_query = final_score_texture.query();
                let final_score_target = Rect::new(
                    (window_size.0 / 2) as i32 - (final_score_texture_query.width / 2) as i32 + 25,
                    (window_size.1 / 2) as i32 - (final_score_texture_query.height / 2) as i32 + 25,
                    final_score_texture_query.width,
                    final_score_texture_query.height,
                );
                canvas.copy(&final_score_texture, None, final_score_target);

                if k_input.shoot || c_input.shoot {
                    main_menu.continue_to_game = false;
                    game = Game::new(window_size, game.texture_creator, game.music_tx);
                    thread::sleep(Duration::new(0, 500000000));
                    let main_menu_sound =
                        Sound::new("assets/Asteroids_MAIN_MENU.mp3", SoundType::Music);
                    game.music_tx.send(main_menu_sound);
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();

        // Calculate FPS every second
        let fps_duration = fps_instant.elapsed().as_millis() as f64 / 1000.0;
        if fps_duration > 1.0 {
            let elapsed = fps_duration;
            frames_per_second = frames / elapsed;
            fps_instant = Instant::now();
            frames = 0.0;
        }

        old_time = now.elapsed();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}

fn get_new_variant(asteroid: &Asteroid) -> Option<AsteroidVariant> {
    match asteroid.variant {
        AsteroidVariant::Large => Some(AsteroidVariant::Medium),
        AsteroidVariant::Medium => Some(AsteroidVariant::Small),
        AsteroidVariant::Small => None,
    }
}
