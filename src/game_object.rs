pub mod game_object {
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;

    #[derive(Debug, Copy, Clone)]
    pub struct KeyboardInput {
        pub forward: bool,
        pub back: bool,
        pub left: bool,
        pub right: bool,
        pub rotate_left: bool,
        pub rotate_right: bool,
        pub shoot: bool,
    }
    #[derive(Debug, Copy, Clone)]
    pub struct ControllerInput {
        pub left: (f32, f32),
        pub right: (f32, f32),
        pub shoot: bool,
    }

    pub trait AsGameObject {
        fn as_game_object(&mut self) -> &mut dyn GameObject;
    }

    pub trait GameObject: AsGameObject {
        fn update(
            &mut self,
            window_size: (u32, u32),
            delta: f64,
            keyboard_input: KeyboardInput,
            controller_input: ControllerInput,
        );
    }

    pub trait AsRenderable {
        fn as_renderable(&mut self) -> &mut dyn Renderable;
    }

    pub trait Renderable: AsRenderable {
        fn get_position(&self) -> Rect;
        fn set_position(&mut self, new_position: Rect);
        fn render(&mut self, render: &mut WindowCanvas);
    }

    pub trait AsGameObjectAndRenderable: AsGameObject + AsRenderable {}
}
