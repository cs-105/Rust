pub mod asteroid {
    use crate::game_object::game_object::GameObject;
    use crate::game_object::game_object::Renderable;

    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::render::WindowCanvas;

    enum AsteroidType {
        Large,
        Medium,
        Small,
    }

    pub struct Asteroid {
        pub texture: Texture,
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
            let rect = Rect::new(50, 50, 150, 150);
            canvas.copy(&self.texture, None, rect).ok();
        }
    }
}
