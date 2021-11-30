pub mod player {
    extern crate sdl2;
    use crate::game_object::game_object::GameObject;
    use crate::game_object::game_object::Renderable;
    use crate::ControllerInput;
    use crate::KeyboardInput;
    use core::f32::consts::PI;
    use glam::Vec2;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::render::WindowCanvas;
    use std::ops::Add;

    const SCREEN_WIDTH: u32 = 1920; //Width in pixels
    const SCREEN_HEIGHT: u32 = 1080; //Height in pixels

    //dimensions of the player sprite
    const PLAYER_SPRITE_WIDTH: u32 = 150; //Width in pixels
    const PLAYER_SPRITE_HEIGHT: u32 = 150; //Height in pixels

    const PLAYER_MOVEMENT_SPEED: f32 = 400.0; //Speed in pixels per second
    const PLAYER_ROTATION_SPEED: f32 = 1.0; //Rotation speed in degrees per second

    const width: f32 = SCREEN_WIDTH as f32 * 1.10;
    const height: f32 = SCREEN_HEIGHT as f32 * 1.10;

    pub struct Player {
        pub texture: Texture,
        pub position: Rect,
        pub pos: Vec2,
        pub angle: f32,
    }
    impl GameObject for Player {
        fn update(
            &mut self,
            delta: f64,
            keyboard_input: KeyboardInput,
            controller_input: ControllerInput,
        ) {
            // Clone position for our new starting point
            let mut new_pos: Vec2 = self.pos.clone();
            let mut new_angle = self.angle.clone();
            let mut force = Vec2::new(0.0, 0.0);

            // Add keyboard forces to vector
            if keyboard_input.forward {
                force = force.add(Vec2::new(0.0, -PLAYER_MOVEMENT_SPEED));
            }
            if keyboard_input.back {
                force = force.add(Vec2::new(0.0, PLAYER_MOVEMENT_SPEED));
            }
            if keyboard_input.left {
                force = force.add(Vec2::new(-PLAYER_MOVEMENT_SPEED, 0.0));
            }
            if keyboard_input.right {
                force = force.add(Vec2::new(PLAYER_MOVEMENT_SPEED, 0.0));
            }

            if keyboard_input.rotate_left {
                new_angle -= (PLAYER_ROTATION_SPEED as f64 * delta) as f32;
            }

            if keyboard_input.rotate_right {
                new_angle += (PLAYER_ROTATION_SPEED as f64 * delta) as f32;
            }

            // Add controller forces to vector
            force = force.add(Vec2::new(
                controller_input.left.0 * PLAYER_MOVEMENT_SPEED,
                controller_input.left.1 * PLAYER_MOVEMENT_SPEED,
            ));

            // If the right controller stick goes passed the deadzone,
            // calculate the angle of the stick using arc tangent
            let y = controller_input.right.1;
            let x = controller_input.right.0;
            if y.abs() > 0.4 || x.abs() > 0.4 {
                new_angle = y.atan2(x);
            }

            // Calculate displacement from forces
            let acceleration = force;
            let velocity = acceleration * Vec2::new(delta as f32, delta as f32);
            let mut position = new_pos + (velocity * Vec2::new(delta as f32, delta as f32));

            // TODO: Run this code on every game object that is a physics object
            if position.x > (width + 50.0) {
                // Right of the screen
                position.x = -40.0;
                position.y = SCREEN_HEIGHT as f32 - position.y - 150.0;
            } else if position.x < -50.0 {
                // Left of the screen
                position.x = width + 40.0;
                position.y = SCREEN_HEIGHT as f32 - position.y - 150.0;
            } else if position.y > (height + 50.0) {
                // Bottom of the screen
                position.y = -40.0;
                position.x = SCREEN_WIDTH as f32 - position.x - 150.0;
            } else if position.y < -50.0 {
                // Top of the screen
                position.y = height + 40.0;
                position.x = SCREEN_WIDTH as f32 - position.x - 150.0;
            }

            self.angle = new_angle;
            self.pos = position;
        }
    }
    impl Renderable for Player {
        fn render(&self, canvas: &mut WindowCanvas) {
            canvas.clear();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            let rect = Rect::new(
                self.pos.x as i32,
                self.pos.y as i32,
                PLAYER_SPRITE_WIDTH,
                PLAYER_SPRITE_HEIGHT,
            );
            canvas
                .copy_ex(
                    &self.texture,
                    None,
                    rect,
                    self.angle as f64 * (180.0 / PI) as f64,
                    None,
                    false,
                    false,
                )
                .ok();
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
}
