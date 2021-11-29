pub mod game_object {
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;

    #[derive(Debug)]
    pub struct KeyboardInput {
        pub forward: bool,
        pub back: bool,
        pub left: bool,
        pub right: bool,
    }
    #[derive(Debug)]
    pub struct ControllerInput {
        pub left: (f32, f32),
        pub right: (f32, f32),
    }

    pub trait GameObject {
        fn update(
            &mut self,
            delta: f64,
            keyboard_input: KeyboardInput,
            controller_input: ControllerInput,
        );
    }

    pub trait Renderable {
        fn set_sprite();
        fn get_sprite();
        fn get_position(&self) -> Rect;
        fn set_position(&mut self, new_position: Rect);
        fn render(&self, render: &mut WindowCanvas);
    }
}
