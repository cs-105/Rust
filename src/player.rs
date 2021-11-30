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

    //dimensions of the player sprite
    const PLAYER_SPRITE_WIDTH: u32 = 150; //Width in pixels
    const PLAYER_SPRITE_HEIGHT: u32 = 150; //Height in pixels

    const PLAYER_MAX_MOVEMENT_SPEED: f32 = 400.0; //Speed in pixels per second
    const PLAYER_ROTATION_SPEED: f32 = 1.0; //Rotation speed in degrees per second
    const PLAYER_ACCELERATION: f32 = 65.0; //Acceleration applied to player

    const DRAG: f32 = 0.075; //Drag multiplier (applied to velocity)

    const width: f32 = 1920.0 * 1.10;
    const height: f32 = 1080.0 * 1.10;

    pub struct Player {
        pub texture: Texture,
        pub position: Rect,
        pub pos: Vec2,
        pub angle: f32,
        pub velocity: Vec2,
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
            let mut new_vel: Vec2 = self.velocity.clone();
            let mut new_angle = self.angle.clone();
            let mut force = Vec2::new(0.0, 0.0);

            // Add keyboard forces to vector
            if keyboard_input.forward {
                force = force.add(Vec2::new(0.0, -PLAYER_ACCELERATION));
            }
            if keyboard_input.back {
                force = force.add(Vec2::new(0.0, PLAYER_ACCELERATION));
            }
            if keyboard_input.left {
                force = force.add(Vec2::new(-PLAYER_ACCELERATION, 0.0));
            }
            if keyboard_input.right {
                force = force.add(Vec2::new(PLAYER_ACCELERATION, 0.0));
            }

            if keyboard_input.rotate_left {
                new_angle -= (PLAYER_ROTATION_SPEED as f64 * delta) as f32;
            }

            if keyboard_input.rotate_right {
                new_angle += (PLAYER_ROTATION_SPEED as f64 * delta) as f32;
            }

            // Add controller forces to vector
            force = force.add(Vec2::new(
                controller_input.left.0 * PLAYER_ACCELERATION,
                controller_input.left.1 * PLAYER_ACCELERATION,
            ));

            // If the right controller stick goes passed the deadzone,
            // calculate the angle of the stick using arc tangent
            let y = controller_input.right.1;
            let x = controller_input.right.0;
            if y.abs() > 0.4 || x.abs() > 0.4 {
                new_angle = y.atan2(x);
            }

            // Calculate velocity from forces
            let mut velocity = new_vel + (force * delta as f32);

            //Clamp velocity
            if velocity.length() > PLAYER_MAX_MOVEMENT_SPEED{

                velocity.x = velocity.x * (PLAYER_MAX_MOVEMENT_SPEED / velocity.length());
                velocity.y = velocity.y * (PLAYER_MAX_MOVEMENT_SPEED / velocity.length());

            }

            //Add drag
            velocity = velocity - (velocity * DRAG);

            //Calculate displacement from velocity
            let mut position = new_pos + (velocity * delta as f32);

            // TODO: Run this code on every game object that is a physics object
            if position.x > (width + 50.0) {
                // Right of the screen
                position.x = -40.0;
                position.y = 1080.0 - position.y - 150.0;
            } else if position.x < -50.0 {
                // Left of the screen
                position.x = width + 40.0;
                position.y = 1080.0 - position.y - 150.0;
            } else if position.y > (height + 50.0) {
                // Bottom of the screen
                position.y = -40.0;
                position.x = 1920.0 - position.x - 150.0;
            } else if position.y < -50.0 {
                // Top of the screen
                position.y = height + 40.0;
                position.x = 1920.0 - position.x - 150.0;
            }

            self.angle = new_angle;
            self.pos = position;
            self.velocity = velocity;
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
