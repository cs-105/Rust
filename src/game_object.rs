trait GameObject {
    fn new() -> Self;

    fn get_position(&self);
    fn set_position(&self);

    fn update(&self, delta_time: f64);
}

struct AsteroidGameMode {
    mouse_position: u128;
    menu_items: Vec<GameObject>
}

impl GameObject for AsteroidGameMode {
    fn update(delta_time: f64) {
        // take mouse input
    }
}

trait Renderable {
    fn set_sprite();
    fn get_sprite();
    fn render(&self, render: &mut WindowCanvas);
}

struct Player {
    velocity: i32,
}

impl GameObject for Player {
    fn new() -> Self {}

    fn update(delta_time: f64) {
        println!();
    }
}

impl Renderable for Player {
    render(canvas: &mut WindowCanvas, color: Color, texture: &Texture, player: &mut Player) {
        
    }
}

impl Player {
    fn set_velocity(velocity: f64) {
        self.velocity = velocity;
    }

    fn get_velocity() -> f64 {
        self.velocity;
    }
}
