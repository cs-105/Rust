pub mod asteroid {
    use crate::game_object::game_object::GameObject;
    use crate::game_object::game_object::Renderable;
    use crate::player::player::PLAYER_SPRITE_WIDTH;
    use crate::ControllerInput;
    use crate::KeyboardInput;
    use glam::Vec2;
    use std::ops::Add;

    use rand::Rng;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::render::WindowCanvas;

    pub enum AsteroidVariant {
        Large,
        Medium,
        Small,
    }

    pub struct Asteroid {
        pub texture: Texture,
        pub pos: Vec2,
        pub angle: f32,
        pub variant: AsteroidVariant,
        pub size: i32,
        pub velocity: Vec2,
    }

    impl Asteroid {
        pub fn new(texture: Texture) -> Self {
            let mut rng = rand::thread_rng();
            Asteroid {
                texture: texture,
                pos: Vec2::new(
                    rng.gen_range(0.0..5000.0 as f32),
                    rng.gen_range(0.0..5000.0 as f32),
                ),
                angle: 0.0,
                variant: AsteroidVariant::Large,
                velocity: Vec2::new(rng.gen_range(-40.0..40.0), rng.gen_range(-40.0..40.0)),
                size: 150,
            }
        }

        pub fn new_with_position(texture: Texture, position: Vec2) -> Self {
            let mut rng = rand::thread_rng();
            Asteroid {
                texture: texture,
                pos: position,
                angle: 0.0,
                variant: AsteroidVariant::Large,
                velocity: Vec2::new(rng.gen_range(-40.0..40.0), rng.gen_range(-40.0..40.0)),
                size: 150,
            }
        }

        pub fn on_collide(&mut self) {
            match self.variant {
                AsteroidVariant::Large => {
                    self.variant = AsteroidVariant::Medium;
                    self.size = 100;
                }
                AsteroidVariant::Medium => {
                    self.variant = AsteroidVariant::Small;
                    self.size = 50;
                }
                AsteroidVariant::Small => {
                    // Do nothing
                }
            }
        }
    }

    impl GameObject for Asteroid {
        fn update(
            &mut self,
            window_size: (u32, u32),
            delta: f64,
            keyboard_input: KeyboardInput,
            controller_input: ControllerInput,
        ) {
            let mut position = self.pos.clone();
            let mut force = Vec2::new(0.0, 0.0);
            force = force.add(self.velocity);

            if position.x > (window_size.0 as f32 + 100.0) {
                // Right of the screen
                position.x = -100.0;
                position.y = window_size.1 as f32 - position.y - PLAYER_SPRITE_WIDTH as f32;
            } else if position.x < -100.0 {
                // Left of the screen
                position.x = window_size.0 as f32 + 100.0;
                position.y = window_size.1 as f32 - position.y - PLAYER_SPRITE_WIDTH as f32;
            } else if position.y > (window_size.1 as f32 + 100.0) {
                // Bottom of the screen
                position.y = -100.0;
                position.x = window_size.0 as f32 - position.x - PLAYER_SPRITE_WIDTH as f32;
            } else if position.y < -100.0 {
                // Top of the screen
                position.y = window_size.1 as f32 + 100.0;
                position.x = window_size.0 as f32 - position.x - PLAYER_SPRITE_WIDTH as f32;
            }

            if self.angle >= 360.0 {
                self.angle = 0.0;
            } else {
                self.angle += 2.0;
            }

            self.pos = position + (force * delta as f32);
        }
    }

    impl Renderable for Asteroid {
        fn set_sprite() {
            todo!()
        }
        fn get_sprite() {
            todo!()
        }
        fn get_position(&self) -> sdl2::rect::Rect {
            Rect::new(
                self.pos.x as i32,
                self.pos.y as i32,
                self.size as u32,
                self.size as u32,
            )
        }
        fn set_position(&mut self, _: sdl2::rect::Rect) {
            todo!()
        }
        fn render(&self, canvas: &mut WindowCanvas) {
            let size = match self.variant {
                AsteroidVariant::Large => 150,
                AsteroidVariant::Medium => 100,
                AsteroidVariant::Small => 50,
            };

            let rect = Rect::new(self.pos.x as i32, self.pos.y as i32, size, size);
            canvas
                .copy_ex(
                    &self.texture,
                    None,
                    rect,
                    self.angle as f64,
                    None,
                    false,
                    false,
                )
                .ok();
        }
    }
}
