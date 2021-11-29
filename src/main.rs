// mod game_object;
// mod graphics;
// mod player;

extern crate sdl2;
use sdl2::controller::Axis;
use sdl2::controller::GameController;
use sdl2::keyboard::Keycode;

use sdl2::event::Event;
use std::collections::HashMap;

use core::iter::{Product, Sum};
use sdl2::image::LoadTexture;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::{PressedScancodeIterator, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use std::collections::HashSet;

use std::time::Instant;

use glam::Vec2;
use std::ops::Add;

use std::f32::consts::PI;

// use graphics::graphics::Graphics;

// use game_object::game_object::Renderable;

// use player::player::Player;

//imports from local crate
// use input::controller::{ , remove_input, Input, Input::*, Player};

//defining constants
//dimensions and title of the window to be rendered
const SCREEN_WIDTH: u32 = 1280; //Width in pixels
const SCREEN_HEIGHT: u32 = 720; //Height in pixels
const WINDOW_TITLE: &str = "The Game";

//dimensions of the player sprite
const PLAYER_SPRITE_WIDTH: u32 = 150; //Width in pixels
const PLAYER_SPRITE_HEIGHT: u32 = 150; //Height in pixels

const PLAYER_MOVEMENT_SPEED: f64 = 5.0; //Speed in pixels per second
const PLAYER_ROTATION_SPEED: f64 = 5.0; //Rotation speed in degrees per second

#[derive(Debug)]
pub struct KeyboardInput {
    forward: bool,
    back: bool,
    left: bool,
    right: bool,
}

#[derive(Debug)]
pub struct ControllerInput {
    left: (f32, f32),
    right: (f32, f32),
}

pub trait Renderable {
    fn set_sprite();
    fn get_sprite();
    fn get_position(&self) -> Rect;
    fn set_position(&mut self, new_position: Rect);
    fn render(&self, render: &mut WindowCanvas);
}

pub trait GameObject {
    fn update(
        &mut self,
        delta: f64,
        keyboard_input: KeyboardInput,
        controller_input: ControllerInput,
    );
}

pub struct Player {
    texture: Texture,
    position: Rect,
    pos: Vec2,
    angle: f32,
}

impl GameObject for Player {
    fn update(
        &mut self,
        delta: f64,
        keyboard_input: KeyboardInput,
        controller_input: ControllerInput,
    ) {
        println!("\ndelta: {:?}", delta);
        println!("kinput: {:?}", keyboard_input);
        println!("cinput: {:?}", controller_input);

        let new_pos: Vec2 = self.pos.clone();
        let speed = 200.0;

        let mut force = Vec2::new(0.0, 0.0);
        if keyboard_input.forward {
            force = force.add(Vec2::new(0.0, -speed));
        }

        if keyboard_input.back {
            force = force.add(Vec2::new(0.0, speed));
        }

        if keyboard_input.left {
            force = force.add(Vec2::new(-speed, 0.0));
        }

        if keyboard_input.right {
            force = force.add(Vec2::new(speed, 0.0));
        }

        force = force.add(Vec2::new(
            controller_input.left.0 * speed,
            controller_input.left.1 * speed,
        ));

        // if (controller_input.right.0 > 0.0 && controller_input.right.1 > 0.0) {
        let y = controller_input.right.1;
        let x = controller_input.right.0;

        if y.abs() > 0.4 || x.abs() > 0.4 {
            self.angle = y.atan2(x);
        }

        let acceleration = force;
        let velocity = acceleration * Vec2::new(delta as f32, delta as f32);
        let position = new_pos + (velocity * Vec2::new(delta as f32, delta as f32));

        self.pos = position;
    }
}

impl Renderable for Player {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let rect = Rect::new(self.pos.x as i32, self.pos.y as i32, 150, 150);
        canvas.copy_ex(
            &self.texture,
            None,
            rect,
            self.angle as f64 * (180.0 / PI) as f64,
            None,
            false,
            false,
        );
        canvas.present();
    }
    fn set_sprite() {
        todo!()
    }
    fn get_sprite() {
        todo!()
    }
    fn get_position(&self) -> Rect {
        self.position
    }
    fn set_position(&mut self, new_position: Rect) {
        self.position = new_position;
    }
}

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

    let mut controller: GameController = (0..available)
        .find_map(|id| {
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
        })
        .expect("Controller");

    let now = Instant::now();
    let mut old_time: Duration = now.elapsed();
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
        };

        let cInput = ControllerInput {
            left: (
                clamp(controller.axis(Axis::LeftX) as f32 / i16::MAX as f32),
                clamp(controller.axis(Axis::LeftY) as f32 / i16::MAX as f32),
            ),
            right: (
                clamp(controller.axis(Axis::RightX) as f32 / i16::MAX as f32),
                clamp(controller.axis(Axis::RightY) as f32 / i16::MAX as f32),
            ),
        };

        let delta_duration = now.elapsed() - old_time;
        let delta_seconds = delta_duration.as_millis() as f64;

        player.update(delta_seconds / 100.0, kInput, cInput);
        player.render(&mut canvas);
        old_time = now.elapsed();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}
