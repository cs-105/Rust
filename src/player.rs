pub mod player {
    extern crate sdl2;
    use crate::game_object::game_object::GameObject;
    use crate::game_object::game_object::Renderable;
    use sdl2::image::LoadTexture;
    use sdl2::rect::{Point, Rect};
    use sdl2::render::Canvas;
    use sdl2::render::{Texture, TextureCreator, WindowCanvas};
    use sdl2::video::WindowContext;
    use std::rc::Rc;

    use crate::graphics::graphics::Graphics;

    pub struct Player {
        texture: String,
        bounds: Rect,
        // position: Point,
    }

    // impl GameObject for Player {}
    impl Renderable for Player {
        fn new() -> Self {
            // let texture = graphics.load_texture("assets/ship.png");

            // let (width, height) = graphics.canvas.output_size().unwrap();
            // let x = width as i32 / 2;
            // let y = height as i32 / 2;

            // let texture = graphics.textures.get("assets/ship.png").unwrap();
            // let texture_size = texture.query();

            // let bounds = Rect::new(
            //     x,
            //     y,
            //     texture_size.width.clone(),
            //     texture_size.height.clone(),
            // );

            // Player {
            //     bounds: bounds,
            //     texture: "assets/ship.png".to_string(),
            // }
        }
        fn set_sprite() {
            todo!()
        }
        fn get_sprite() {
            todo!()
        }
        fn render<'a>(&self) {
            // let screen_position = Point::new(self.bounds.x(), self.bounds.y());
            // let screen_rect =
            //     Rect::from_center(screen_position, self.bounds.width(), self.bounds.height());

            // let texture = graphics.textures.get("assets/ship.png").unwrap();

            // let texture = graphics
            //     .canvas
            //     .copy_ex(texture, self.bounds, screen_rect, 0.0, None, false, false)
            //     .unwrap();
        }
        fn get_position(&self) {
            todo!()
        }
        fn set_position(&self) {
            todo!()
        }
    }
}
