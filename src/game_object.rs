pub mod game_object {
    extern crate sdl2;

    use crate::graphics::graphics::Graphics;
    use sdl2::render::{TextureCreator, WindowCanvas};
    use sdl2::video::WindowContext;
    use std::rc::Rc;
    pub trait GameObject {
        fn new() -> Self;
        fn update(&self, delta_time: f64);
    }
    pub trait Renderable {
        fn new<'a>(graphics: &'a Graphics<'a>) -> Self;
        fn set_sprite();
        fn get_sprite();
        fn get_position(&self);
        fn set_position(&self);
        fn render<'a>(&self, render: &'a Graphics);
    }
}
