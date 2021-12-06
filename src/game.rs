pub mod game {
    use crate::game_object::game_object::AsGameObject;
    use crate::game_object::game_object::AsRenderable;
    use crate::get_new_variant;
    use crate::is_colliding;
    use crate::player::player::draw_point;
    use crate::render_text;
    use crate::set_vec_angle;
    use crate::transform_to_ship_space;
    use crate::Asteroid;
    use crate::Bullet;
    use crate::ControllerInput;
    use crate::GameObject;
    use crate::KeyboardInput;
    use crate::Player;
    use crate::Renderable;
    use crate::PLAYER_SPRITE_HEIGHT;
    use crate::PLAYER_SPRITE_WIDTH;
    use crate::{Sound, SoundType};
    use glam::Vec2;
    use sdl2::image::LoadTexture;
    use sdl2::rect::Rect;
    use sdl2::render::TextureCreator;
    use sdl2::ttf::Font;
    use std::sync::mpsc::Sender;
    use std::time::Instant;

    pub struct Game<T> {
        asteroids: Vec<Asteroid>,
        bullets: Vec<Bullet>,
        player: Player,

        previous_shoot: bool,
        died_at: Instant,

        pub dead: bool,
        pub finished: bool,
        pub score: i32,
        pub texture_creator: TextureCreator<T>,

        window_size: (u32, u32),
        pub music_tx: Sender<Sound>,
    }

    impl<T> Game<T> {
        pub fn new(
            window_size: (u32, u32),
            texture_creator: TextureCreator<T>,
            music_tx: Sender<Sound>,
        ) -> Self {
            let mut game: Game<T> = Game {
                asteroids: Vec::new(),
                bullets: Vec::new(),
                player: Player {
                    texture: texture_creator.load_texture("assets/ship.png").unwrap(),
                    position: Rect::new(500, 500, 150, 150),
                    pos: Vec2::new(
                        window_size.0 as f32 / 2.0 - 150.0 / 2.0,
                        window_size.1 as f32 / 2.0 - 150.0 / 2.0,
                    ),
                    angle: 0.0,
                    velocity: Vec2::new(0.0, 0.0),
                    previous_shoot: false,
                },

                previous_shoot: false,
                finished: false,
                dead: false,
                score: 0,
                died_at: Instant::now(),
                window_size: window_size,
                texture_creator: texture_creator,
                music_tx: music_tx,
            };

            for n in 1..20 {
                game.asteroids.push(Asteroid::new(
                    game.texture_creator
                        .load_texture("assets/asteroid.png")
                        .unwrap(),
                ));
            }

            game
        }

        pub fn start(&mut self) {
            let game_sound = Sound::new("assets/Asteroids_GAME.mp3", SoundType::Music);
            self.music_tx.send(game_sound);
        }
    }

    impl<T> AsGameObject for Game<T> {
        fn as_game_object(&mut self) -> &mut dyn GameObject {
            self
        }
    }

    impl<T> GameObject for Game<T> {
        fn update(
            &mut self,
            window_size: (u32, u32),
            delta: f64,
            keyboard_input: KeyboardInput,
            controller_input: ControllerInput,
        ) {
            self.window_size = window_size;

            if !self.dead {
                self.player
                    .update(window_size, delta, keyboard_input, controller_input);
                for asteroid in self.asteroids.iter_mut() {
                    asteroid.update(window_size, delta, keyboard_input, controller_input);
                }
            } else if self.died_at.elapsed().as_secs() >= 1 {
                self.dead = false;
            }

            let shoot = keyboard_input.shoot || controller_input.shoot;
            if !self.previous_shoot && shoot && !self.dead {
                let bullet_sound = Sound::new("assets/Laser_Sound.mp3", SoundType::SoundEffect);
                self.music_tx.send(bullet_sound);

                let bullet_vec = Vec2::new(PLAYER_SPRITE_WIDTH as f32 / 2.0, 0.0);
                let bullet_rect = set_vec_angle(bullet_vec, self.player.angle);
                let ship_width = PLAYER_SPRITE_WIDTH as f32;
                let ship_height = PLAYER_SPRITE_HEIGHT as f32;

                // A
                let ship_center = Vec2::new(
                    self.player.pos.x + ship_width / 2.0,
                    self.player.pos.y + ship_height / 2.0,
                );

                // B
                let bullet_final = transform_to_ship_space(&self.player, bullet_rect);

                // Point slope form
                let run = -(ship_center.x - bullet_final.x);
                let rise = -(ship_center.y - bullet_final.y);
                let bullet = Bullet {
                    pos: bullet_final,
                    velocity: Vec2::new(run * 2.0, rise * 2.0) + self.player.velocity,
                };
                self.bullets.push(bullet);
            }
            self.previous_shoot = shoot;
            if !self.dead {
                for bullet in self.bullets.iter_mut() {
                    bullet.update(window_size, delta, keyboard_input, controller_input);
                }
            } else {
                self.bullets = Vec::new();
            }

            // Clone memory
            let mut asteroids = std::mem::take(&mut self.asteroids);
            let mut bullets = std::mem::take(&mut self.bullets);

            let mut asteroids_to_create: Vec<Asteroid> = Vec::new();
            asteroids.retain(|asteroid| {
                let mut retain_asteroid = true;
                if is_colliding(&self.player, asteroid) {
                    self.player.pos = Vec2::new(
                        window_size.0 as f32 / 2.0 - 150.0 / 2.0,
                        window_size.1 as f32 / 2.0 - 150.0 / 2.0,
                    );
                    self.player.velocity = Vec2::new(0.0, 0.0);
                    self.player.angle = 0.0;
                    self.died_at = Instant::now();
                    self.dead = true;
                    let explosion_sound =
                        Sound::new("assets/Explosion.mp3", SoundType::SoundEffect);
                    self.music_tx.send(explosion_sound);
                    match get_new_variant(asteroid) {
                        None => {}
                        Some(new_variant) => {
                            let mut new_asteroid = Asteroid::new_with_position(
                                self.texture_creator
                                    .load_texture("assets/asteroid.png")
                                    .unwrap(),
                                asteroid.pos,
                            );
                            new_asteroid.variant = new_variant;
                            asteroids_to_create.push(new_asteroid);
                        }
                    };
                    match get_new_variant(asteroid) {
                        None => {}
                        Some(new_variant) => {
                            let mut new_asteroid = Asteroid::new_with_position(
                                self.texture_creator
                                    .load_texture("assets/asteroid.png")
                                    .unwrap(),
                                asteroid.pos,
                            );
                            new_asteroid.variant = new_variant;
                            asteroids_to_create.push(new_asteroid);
                        }
                    };
                    // Destroy asteroid
                    retain_asteroid = false;
                }
                bullets.retain(|bullet| {
                    let mut retain_bullet = true;
                    if is_colliding(bullet, asteroid) {
                        let explosion_sound =
                            Sound::new("assets/Explosion.mp3", SoundType::SoundEffect);
                        self.music_tx.send(explosion_sound);
                        match get_new_variant(asteroid) {
                            None => {}
                            Some(new_variant) => {
                                let mut new_asteroid = Asteroid::new_with_position(
                                    self.texture_creator
                                        .load_texture("assets/asteroid.png")
                                        .unwrap(),
                                    asteroid.pos,
                                );
                                new_asteroid.variant = new_variant;
                                asteroids_to_create.push(new_asteroid);
                            }
                        };
                        match get_new_variant(asteroid) {
                            None => {}
                            Some(new_variant) => {
                                let mut new_asteroid = Asteroid::new_with_position(
                                    self.texture_creator
                                        .load_texture("assets/asteroid.png")
                                        .unwrap(),
                                    asteroid.pos,
                                );
                                new_asteroid.variant = new_variant;
                                asteroids_to_create.push(new_asteroid);
                            }
                        };
                        self.score += 10;
                        // Destroy both
                        retain_asteroid = false;
                        retain_bullet = false;
                    }
                    retain_bullet
                });

                retain_asteroid
            });

            self.asteroids = asteroids;
            self.bullets = bullets;
            self.asteroids.extend(asteroids_to_create);

            if self.asteroids.len() < 1 {
                self.finished = true;
            }
        }
    }

    impl<T> AsRenderable for Game<T> {
        fn as_renderable(&mut self) -> &mut dyn Renderable {
            self
        }
    }

    impl<T> Renderable for Game<T> {
        fn get_position(&self) -> sdl2::rect::Rect {
            // No position, crash if called
            todo!()
        }
        fn set_position(&mut self, _: sdl2::rect::Rect) {
            // No position, crash if called
            todo!()
        }

        fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
            if !self.dead {
                self.player.render(canvas);
            }

            for bullet in self.bullets.iter_mut() {
                bullet.render(canvas);
            }

            for asteroid in self.asteroids.iter_mut() {
                asteroid.render(canvas);
            }
        }
    }
}
