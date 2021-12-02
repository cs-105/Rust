pub mod asteroid {
    use crate::game_object::game_object::GameObject;
    use crate::game_object::game_object::Renderable;
    use crate::player::player::PLAYER_SPRITE_WIDTH;
    use crate::player::player::SCREEN_HEIGHT;
    use crate::player::player::SCREEN_WIDTH;
    use crate::ControllerInput;
    use crate::KeyboardInput;
    use glam::Vec2;
    use std::ops::Add;

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
    }

    impl Asteroid {
        pub fn new(texture: Texture) -> Self {
            Asteroid {
                texture: texture,
                pos: Vec2::new(0.0, 0.0),
                angle: 0.0,
                variant: AsteroidVariant::Large,
            }
        }

        pub fn new_with_position(texture: Texture, position: Vec2) -> Self {
            Asteroid {
                texture: texture,
                pos: position,
                angle: 0.0,
                variant: AsteroidVariant::Large,
            }
        }
    }

    impl GameObject for Asteroid {
        fn update(
            &mut self,
            delta: f64,
            keyboard_input: KeyboardInput,
            controller_input: ControllerInput,
        ) {
            let mut position = self.pos.clone();
            let mut force = Vec2::new(0.0, 0.0);
            force = force.add(Vec2::new(40.0, 30.0));

            if position.x > (SCREEN_WIDTH as f32 + 100.0) {
                // Right of the screen
                position.x = -100.0;
                position.y = SCREEN_HEIGHT as f32 - position.y - PLAYER_SPRITE_WIDTH as f32;
            } else if position.x < -100.0 {
                // Left of the screen
                position.x = SCREEN_WIDTH as f32 + 100.0;
                position.y = SCREEN_HEIGHT as f32 - position.y - PLAYER_SPRITE_WIDTH as f32;
            } else if position.y > (SCREEN_HEIGHT as f32 + 100.0) {
                // Bottom of the screen
                position.y = -100.0;
                position.x = SCREEN_WIDTH as f32 - position.x - PLAYER_SPRITE_WIDTH as f32;
            } else if position.y < -100.0 {
                // Top of the screen
                position.y = SCREEN_HEIGHT as f32 + 100.0;
                position.x = SCREEN_WIDTH as f32 - position.x - PLAYER_SPRITE_WIDTH as f32;
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
            todo!()
        }
        fn set_position(&mut self, _: sdl2::rect::Rect) {
            todo!()
        }
        fn render(&self, canvas: &mut WindowCanvas) {
            let rect = Rect::new(self.pos.x as i32, self.pos.y as i32, 150, 150);
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
