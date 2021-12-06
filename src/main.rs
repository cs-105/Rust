mod asteroid;
mod collide;
mod game_object;
mod graphics;
mod input;
mod music;
mod player;

extern crate sdl2;
use crate::asteroid::asteroid::Asteroid;
use crate::asteroid::asteroid::AsteroidVariant;
use crate::collide::collide::is_colliding;
use crate::game_object::game_object::ControllerInput;
use crate::game_object::game_object::GameObject;
use crate::game_object::game_object::KeyboardInput;
use crate::game_object::game_object::Renderable;
use crate::graphics::graphics::render_text;
use crate::player::player::set_vec_angle;
use crate::player::player::transform_to_ship_space;
use crate::player::player::Bullet;
use crate::player::player::Player;
use crate::player::player::PLAYER_SPRITE_HEIGHT;
use crate::player::player::PLAYER_SPRITE_WIDTH;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
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
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

//imports from music file
use crate::music::music::in_game_music;
use crate::music::music::main_menu_music;
use crate::music::music::Sound;

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
    let music_thread = thread::spawn(move || main_menu_music(rx));
    let sound = Sound::new("assets/Asteroids_GAME.mp3");

    tx.send(sound);

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

    let mut previous_shoot = false;
    let mut bullets: Vec<Bullet> = Vec::new();

    let mut frames: f64 = 0.0;
    let mut frames_per_second: f64 = 0.0;

    let mut score = 0;

    let mut died_at = Instant::now();
    let mut dead = false;

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
            left: keys.get(&Keycode::A).is_some(),
            right: keys.get(&Keycode::D).is_some(),

            rotate_left: keys.get(&Keycode::Q).is_some(),
            rotate_right: keys.get(&Keycode::E).is_some(),

            shoot: keys.get(&Keycode::Space).is_some(),
        };

        let c_input: ControllerInput;

        if let Some(c) = &controller {
            c_input = ControllerInput {
                left: (
                    // clamp(c.axis(Axis::LeftY) as f32 / i16::MAX as f32),
                    0.0,
                    clamp(c.axis(Axis::LeftY) as f32 / i16::MAX as f32),
                ),
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

        if !dead {
            player.update(window_size, delta_seconds / 100.0, k_input, c_input);
            player.render(&mut canvas);

            for asteroid in asteroids.iter_mut() {
                asteroid.update(window_size, delta_seconds / 100.0, k_input, c_input);
            }
        } else if died_at.elapsed().as_secs() >= 1 {
            dead = false;
        }

        for asteroid in asteroids.iter() {
            asteroid.render(&mut canvas);
        }

        let shoot = k_input.shoot || c_input.shoot;
        if !previous_shoot && shoot && !dead {
            let bullet_vec = Vec2::new(PLAYER_SPRITE_WIDTH as f32 / 2.0, 0.0);
            let bullet_rect = set_vec_angle(bullet_vec, player.angle);

            let ship_width = PLAYER_SPRITE_WIDTH as f32;
            let ship_height = PLAYER_SPRITE_HEIGHT as f32;

            // A
            let ship_center = Vec2::new(
                player.pos.x + ship_width / 2.0,
                player.pos.y + ship_height / 2.0,
            );

            // B
            let bullet_final = transform_to_ship_space(&player, bullet_rect);

            let run = -(ship_center.x - bullet_final.x);
            let rise = -(ship_center.y - bullet_final.y);

            let bullet = Bullet {
                pos: bullet_final,
                velocity: Vec2::new(run * 2.0, rise * 2.0) + player.velocity,
            };
            bullets.push(bullet);
        }
        previous_shoot = shoot;

        if !dead {
            for bullet in bullets.iter_mut() {
                bullet.update(window_size, delta_seconds / 100.0, k_input, c_input);
                bullet.render(&mut canvas);
            }
        } else {
            bullets = Vec::new();
        }

        let mut asteroids_to_create: Vec<Asteroid> = Vec::new();
        asteroids.retain(|asteroid| {
            let mut retain_asteroid = true;

            if is_colliding(&player, asteroid) {
                println!("Collding!");
                player.pos = Vec2::new(
                    window_size.0 as f32 / 2.0 - 150.0 / 2.0,
                    window_size.1 as f32 / 2.0 - 150.0 / 2.0,
                );
                player.velocity = Vec2::new(0.0, 0.0);
                player.angle = 0.0;
                died_at = Instant::now();
                dead = true;

                match get_new_variant(asteroid) {
                    None => {}
                    Some(new_variant) => {
                        let mut new_asteroid = Asteroid::new_with_position(
                            texture_creator.load_texture("assets/asteroid.png").unwrap(),
                            asteroid.pos,
                        );
                        new_asteroid.variant = new_variant;
                        asteroids_to_create.push(new_asteroid);
                    }
                };

                match get_new_variant(asteroid) {
                    None => {}
                    Some(new_variant) => {
                        let mut new_asteroid = Asteroid::new_with_position(
                            texture_creator.load_texture("assets/asteroid.png").unwrap(),
                            asteroid.pos,
                        );
                        new_asteroid.variant = new_variant;
                        asteroids_to_create.push(new_asteroid);
                    }
                };

                // Destroy asteroid
                retain_asteroid = false;
            }

            bullets.retain(|bullet| {
                let mut retain_bullet = true;
                if is_colliding(bullet, asteroid) {
                    println!("COLLIDE");
                    match get_new_variant(asteroid) {
                        None => {}
                        Some(new_variant) => {
                            let mut new_asteroid = Asteroid::new_with_position(
                                texture_creator.load_texture("assets/asteroid.png").unwrap(),
                                asteroid.pos,
                            );
                            new_asteroid.variant = new_variant;
                            asteroids_to_create.push(new_asteroid);
                        }
                    };

                    match get_new_variant(asteroid) {
                        None => {}
                        Some(new_variant) => {
                            let mut new_asteroid = Asteroid::new_with_position(
                                texture_creator.load_texture("assets/asteroid.png").unwrap(),
                                asteroid.pos,
                            );
                            new_asteroid.variant = new_variant;
                            asteroids_to_create.push(new_asteroid);
                        }
                    };

                    score += 10;

                    // Destroy both
                    retain_asteroid = false;
                    retain_bullet = false;
                }
                retain_bullet
            });

            retain_asteroid
        });

        asteroids.extend(asteroids_to_create);

        canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));

        let score_texture = render_text(
            format!("score: {}", score),
            &font,
            &mut canvas,
            &texture_creator,
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
            &mut canvas,
            &texture_creator,
        )?;

        let fps_texture_query = fps_texture.query();
        let fps_target = Rect::new(0, 0, fps_texture_query.width, fps_texture_query.height);
        canvas.copy(&fps_texture, None, fps_target);

        if dead {
            let dead_texture =
                render_text("DEAD".to_string(), &font, &mut canvas, &texture_creator)?;
            let dead_texture_query = dead_texture.query();
            let dead_target = Rect::new(
                (window_size.0 / 2) as i32 - (dead_texture_query.width / 2) as i32,
                (window_size.1 / 2) as i32 - (dead_texture_query.height / 2) as i32,
                dead_texture_query.width,
                dead_texture_query.height,
            );
            canvas.copy(&dead_texture, None, dead_target);
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
