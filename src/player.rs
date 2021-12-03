pub mod player {
    extern crate sdl2;
    use crate::game_object::game_object::GameObject;
    use crate::game_object::game_object::Renderable;
    use crate::ControllerInput;
    use crate::KeyboardInput;
    use core::f32::consts::PI;
    use glam::Vec2;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::render::WindowCanvas;
    use std::ops::Add;

    pub const SCREEN_WIDTH: u32 = 1280; //Width in pixels
    pub const SCREEN_HEIGHT: u32 = 720; //Height in pixels

    //dimensions of the player sprite
    pub const PLAYER_SPRITE_WIDTH: u32 = 75; //Width in pixels
    pub const PLAYER_SPRITE_HEIGHT: u32 = 58; //Height in pixels

    pub const PLAYER_MAX_MOVEMENT_SPEED: f32 = 400.0; //Speed in pixels per second
    pub const PLAYER_ROTATION_SPEED: f32 = 0.5; //Rotation speed in radians per second
    pub const PLAYER_ACCELERATION: f32 = 65.0; //Acceleration applied to player

    const DRAG: f32 = 0.075; //Drag multiplier (applied to velocity)

    const width: f32 = SCREEN_WIDTH as f32 * 1.10;
    const height: f32 = SCREEN_HEIGHT as f32 * 1.10;

    pub struct Bullet {
        pub pos: Vec2,
        pub velocity: Vec2,
    }

    impl GameObject for Bullet {
        fn update(
            &mut self,
            delta: f64,
            keyboard_input: KeyboardInput,
            controller_input: ControllerInput,
        ) {
            self.pos = self.pos + (self.velocity * delta as f32);
        }
    }

    impl Renderable for Bullet {
        fn set_sprite() {
            todo!()
        }
        fn get_sprite() {
            todo!()
        }
        fn set_position(&mut self, _: sdl2::rect::Rect) {
            todo!()
        }
        fn get_position(&self) -> sdl2::rect::Rect {
            todo!()
        }
        fn render(&self, canvas: &mut WindowCanvas) {
            draw_point(canvas, self.pos);
        }
    }

    pub struct Player {
        pub texture: Texture,
        pub position: Rect,
        pub pos: Vec2,
        pub angle: f32,
        pub velocity: Vec2,
        pub bullets: Vec<Bullet>,
        pub previous_shoot: bool,
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

            // Transform force
            force = set_vec_angle(force, new_angle + (90.0 * PI / 180.0));

            // Calculate velocity from forces
            let mut velocity = new_vel + (force * delta as f32);

            //Clamp velocity
            if velocity.length() > PLAYER_MAX_MOVEMENT_SPEED {
                velocity.x = velocity.x * (PLAYER_MAX_MOVEMENT_SPEED / velocity.length());
                velocity.y = velocity.y * (PLAYER_MAX_MOVEMENT_SPEED / velocity.length());
            }

            //Add drag
            velocity = velocity - (velocity * DRAG);

            //Calculate displacement from velocity
            let mut position = new_pos + (velocity * delta as f32);

            // TODO: Run this code on every game object that is a physics object
            if position.x > (width + 80.0) {
                // Right of the screen
                position.x = -80.0;
                position.y = SCREEN_HEIGHT as f32 - position.y - PLAYER_SPRITE_WIDTH as f32;
            } else if position.x < -80.0 {
                // Left of the screen
                position.x = width + 80.0;
                position.y = SCREEN_HEIGHT as f32 - position.y - PLAYER_SPRITE_WIDTH as f32;
            } else if position.y > (height + 80.0) {
                // Bottom of the screen
                position.y = -80.0;
                position.x = SCREEN_WIDTH as f32 - position.x - PLAYER_SPRITE_WIDTH as f32;
            } else if position.y < -80.0 {
                // Top of the screen
                position.y = height + 80.0;
                position.x = SCREEN_WIDTH as f32 - position.x - PLAYER_SPRITE_WIDTH as f32;
            }

            let shoot = keyboard_input.shoot || controller_input.shoot;

            if !self.previous_shoot && shoot {
                println!("shoot!");
                let bullet_vec = Vec2::new(PLAYER_SPRITE_WIDTH as f32 / 2.0, 0.0);
                let bullet_rect = set_vec_angle(bullet_vec, self.angle);

                let ship_width = PLAYER_SPRITE_WIDTH as f32;
                let ship_height = PLAYER_SPRITE_HEIGHT as f32;

                // A
                let ship_center = Vec2::new(
                    self.pos.x + ship_width / 2.0,
                    self.pos.y + ship_height / 2.0,
                );

                // B
                let bullet_final = transform_to_ship_space(self, bullet_rect);

                let run = -(ship_center.x - bullet_final.x);
                let rise = -(ship_center.y - bullet_final.y);

                let bullet = Bullet {
                    pos: bullet_final,
                    velocity: Vec2::new(run * 2.0, rise * 2.0) + velocity,
                };
                self.bullets.push(bullet);
            }
            self.previous_shoot = shoot;

            self.angle = new_angle;
            self.pos = position;
            self.velocity = velocity;
        }
    }
    impl Renderable for Player {
        fn render(&self, canvas: &mut WindowCanvas) {
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

    fn transform_to_ship_space(player: &Player, vec: Vec2) -> Vec2 {
        let ship_width = PLAYER_SPRITE_WIDTH as f32;
        let ship_height = PLAYER_SPRITE_HEIGHT as f32;
        Vec2::new(
            vec.x + player.pos.x + ship_width / 2.0,
            vec.y + player.pos.y + ship_height / 2.0,
        )
    }

    fn draw_point(canvas: &mut WindowCanvas, pos: Vec2) {
        let scale = 10.0;
        canvas.set_scale(scale, scale);
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        canvas.draw_point(Point::new(
            pos.x as i32 / scale as i32,
            pos.y as i32 / scale as i32,
        ));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.set_scale(1.0, 1.0);
    }

    fn set_vec_angle(vector: Vec2, angle: f32) -> Vec2 {
        let new_x = vector.x * angle.cos() - vector.y * angle.sin();
        let new_y = vector.x * angle.sin() + vector.y * angle.cos();

        Vec2::new(new_x, new_y)
    }
}
