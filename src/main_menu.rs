pub mod main_menu {
    use crate::game_object::game_object::AsGameObject;
    use crate::game_object::game_object::AsGameObjectAndRenderable;
    use crate::game_object::game_object::AsRenderable;
    use crate::render_text;
    use crate::Asteroid;
    use crate::ControllerInput;
    use crate::GameObject;
    use crate::KeyboardInput;
    use crate::Renderable;
    use sdl2::image::LoadTexture;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::render::TextureCreator;
    use sdl2::render::TextureQuery;
    use sdl2::ttf::Font;

    pub struct MainMenu {
        game_objects: Vec<Box<dyn AsGameObjectAndRenderable>>,
        pub continue_to_game: bool,
        asteroids_text: (Texture, Rect),
        play_text: (Texture, Rect),
        credits_text: (Texture, Rect),
    }

    impl MainMenu {
        pub fn new<T>(
            window_size: (u32, u32),
            font: &Font,
            texture_creator: &TextureCreator<T>,
        ) -> Self {
            let mut game_objects: Vec<Box<dyn AsGameObjectAndRenderable>> = Vec::new();

            for _ in 1..100 {
                let texture = texture_creator.load_texture("assets/asteroid.png").unwrap();
                let asteroid = Box::new(Asteroid::new(texture));
                game_objects.push(asteroid);
            }

            let asteroids_texture =
                render_text("Welcome to Asteroids".to_string(), &font, &texture_creator).unwrap();
            let asteroids_texture_query = asteroids_texture.query();
            let asteroids_target = Rect::new(
                (window_size.0 / 2) as i32 - (asteroids_texture_query.width / 2) as i32,
                (window_size.1 / 2) as i32 - (asteroids_texture_query.height / 2) as i32 - 25,
                asteroids_texture_query.width,
                asteroids_texture_query.height,
            );

            let play_texture =
                render_text("Press SHOOT to play".to_string(), &font, &texture_creator).unwrap();
            let play_texture_query = play_texture.query();
            let play_target = Rect::new(
                (window_size.0 / 2) as i32 - (play_texture_query.width / 2) as i32,
                (window_size.1 / 2) as i32 - (play_texture_query.height / 2) as i32 + 25,
                play_texture_query.width,
                play_texture_query.height,
            );

            let credits_texture = render_text(
                "By John Panos, Todd Knight, and William Mitsuk".to_string(),
                &font,
                &texture_creator,
            )
            .unwrap();
            let credits_texture_query = credits_texture.query();
            let credits_target = Rect::new(
                (window_size.0 / 2) as i32 - (credits_texture_query.width / 2) as i32,
                (window_size.1) as i32 - credits_texture_query.height as i32 - 25,
                credits_texture_query.width,
                credits_texture_query.height,
            );

            MainMenu {
                game_objects: game_objects,
                continue_to_game: false,
                asteroids_text: (asteroids_texture, asteroids_target),
                play_text: (play_texture, play_target),
                credits_text: (credits_texture, credits_target),
            }
        }
    }

    impl AsGameObject for MainMenu {
        fn as_game_object(&mut self) -> &mut dyn GameObject {
            self
        }
    }

    impl GameObject for MainMenu {
        fn update(
            &mut self,
            window_size: (u32, u32),
            delta: f64,
            keyboard_input: KeyboardInput,
            controller_input: ControllerInput,
        ) {
            for renderable in self.game_objects.iter_mut() {
                renderable.as_game_object().update(
                    window_size,
                    delta,
                    keyboard_input,
                    controller_input,
                );
            }

            if keyboard_input.shoot || controller_input.shoot {
                self.continue_to_game = true;
            }
        }
    }

    impl AsRenderable for MainMenu {
        fn as_renderable(&mut self) -> &mut dyn Renderable {
            self
        }
    }

    impl Renderable for MainMenu {
        fn get_position(&self) -> sdl2::rect::Rect {
            todo!()
        }
        fn set_position(&mut self, _: sdl2::rect::Rect) {
            todo!()
        }
        fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
            for renderable in self.game_objects.iter_mut() {
                renderable.as_renderable().render(canvas);
            }

            canvas.copy(&self.asteroids_text.0, None, self.asteroids_text.1);
            canvas.copy(&self.play_text.0, None, self.play_text.1);
            canvas.copy(&self.credits_text.0, None, self.credits_text.1);
        }
    }
}
