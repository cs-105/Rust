// pub mod controller{

// 	extern crate sdl2;

// 	use sdl2::rect::{Rect, Point};
// 	use sdl2::render::Texture;
// 	use sdl2::EventPump;

// 	use crate::SCREEN_WIDTH;
// 	use crate::SCREEN_HEIGHT;

// 	//enum that defines the different directions the player can move in
// 	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// 	pub enum Input{

// 		Up,
// 		Down,
// 		Left,
// 		Right,
// 		RotateLeft,
// 		RotateRight,

// 	}

// 	//Physics vector, mainly used for storing velocity
// 	#[derive(Debug, Clone, Copy)]
// 	pub struct Vector{

// 	    pub magnitude: f64,
// 	    pub direction: f64,

// 	}

// 	//GameObject Struct defines characterstics of non player objects
// 	pub struct GameObject<'r>{

// 		position: Point, //2-D Cartesian Point
// 		sprite: Rect, //Dimensions for selecting image from spritesheet
// 		texture: Texture<'r>, //Texture for rendering
// 		velocity: Vector, //Velocity vector

// 	}

// 	impl<'r> GameObject<'_>{

// 		//getters
// 		pub fn get_position(&mut self) -> Point {self.position}
// 		pub fn get_sprite(&mut self) -> Rect {self.sprite}
// 		pub fn get_speed(&mut self) -> f64 {self.velocity.magnitude}
// 		pub fn get_heading(&mut self) -> f64 {self.velocity.direction}
// 		pub fn get_velocity(&mut self) -> Vector {self.velocity}

// 		//setters
// 		pub fn set_position(&mut self, new_position: Point){self.position = new_position}
// 		pub fn set_sprite(&mut self, new_sprite: Rect){self.sprite = new_sprite}
// 		pub fn set_speed(&mut self, new_speed: f64){self.velocity.magnitude = new_speed}
// 		pub fn set_heading(&mut self, new_heading: f64){self.velocity.direction = new_heading}
// 		pub fn set_velocity(&mut self, new_velocity: Vector){self.velocity = new_velocity}

// 		//constructor
// 		pub fn create_game_object(position: Point, sprite: Rect, texture: Texture<'r>, velocity: Vector) -> GameObject<'static>{

// 			GameObject<'static>{

// 				position,
// 				sprite,
// 				texture,
// 				velocity,
// 			}

// 		}

// 	}

// 	//Player Struct keeps track of data about the player avatar
// 	pub struct Player<'r>{

// 		game_object: GameObject<'r>, //GameObject used for rendering and updating
// 	 	rotation_speed: f64, //Rate at which the player sprite is rotated
// 	    heading: f64, //Heading of the player

// 	}

// 	//Player constructor
// 	pub fn create_player<'r>(position: Point, sprite: Rect, speed: f64, texture: Texture<'r>, rotation_speed: f64, heading: f64) -> Player<'static>{

// 		Player {

// 			game_object: create_game_object(position, sprite, texture, Vector{magnitude: 0.0, direction: normalize_heading(heading)}),
// 			rotation_speed,
// 			heading: normalize_heading(heading),

// 		}

// 	}

// 	pub trait Update{

// 		fn update(&self, event_pump: EventPump);

// 	}

// 	impl Update for Player{

// 		fn update(&self, state: Keyboard_State){

// 			if(state.is_scancode_pressed(Scancode::W)){

// 			}

// 		}

// 	}

// 	//Player functions
// 	impl Player{

// 		//getters
// 		pub fn get_game_object(&mut self) -> GameObject {self.game_object}
// 		pub fn get_position(&mut self) -> Point {self.game_object.get_position()}
// 		pub fn get_sprite(&mut self) -> Rect {self.game_object.get_sprite()}
// 		pub fn get_speed(&mut self) -> f64 {self.game_object.get_speed()}
// 		pub fn get_rotation_speed(&mut self) -> f64 {self.rotation_speed}
// 		pub fn get_heading(&mut self) -> f64 {self.heading}

// 		//setters
// 		pub fn set_game_object(&mut self, new_game_object: GameObject<'static>){self.game_object = new_game_object;}
// 		pub fn set_position(&mut self, new_position: Point) {self.game_object.set_position(new_position);}
// 		pub fn set_sprite(&mut self, new_sprite: Rect) {self.game_object.set_sprite(new_sprite);}
// 		pub fn set_speed(&mut self, new_speed: f64) {self.game_object.set_speed(new_speed)}
// 		pub fn set_rotation_speed(&mut self, new_speed: f64) {self.rotation_speed = new_speed}
// 		pub fn set_heading(&mut self, new_heading: f64) {self.heading = new_heading}

// 		//functions
// 		// pub fn update_player(&mut self, inputs: &mut Vec<Input>){

// 		// 	use Input::*;

// 		//     //velocity vectors relative to the player's heading
// 		//     let mut velocity_x = Vector{

// 		//         magnitude: 0.0,
// 		//         direction: 0.0,

// 		//     };
// 		//     let mut velocity_y = Vector{

// 		//         magnitude: 0.0,
// 		//         direction: 0.0,

// 		//     };

// 		//     for input in inputs.iter(){

// 		//         match input{

// 		//             Up => {

// 		//                 velocity_y.magnitude = self.speed as f64;
// 		//                 velocity_y.direction = 0.0;

// 		//             },
// 		//             Down => {

// 		//                 velocity_y.magnitude = self.speed as f64;
// 		//                 velocity_y.direction = 180.0;

// 		//             },
// 		//             Left => {

// 		//                 velocity_x.magnitude = self.speed as f64;
// 		//                 velocity_x.direction = 90.0;

// 		//             },
// 		//             Right => {

// 		//                 velocity_x.magnitude = self.speed as f64;
// 		//                 velocity_x.direction = 270.0;

// 		//             },
// 		//             RotateRight => {

// 		//             	self.heading = normalize_heading(self.heading + self.rotation_speed as f64);

// 		//             },
// 		//           	RotateLeft => {

// 		//             	self.heading = normalize_heading(self.heading - self.rotation_speed as f64);

// 		//             },

// 		//         };

// 		//     }

// 		//     let (offset_x, offset_y) = transform_vector(velocity_x, velocity_y, self.heading);

// 		//     self.position = self.position.offset(offset_x.trunc() as i32, offset_y.trunc() as i32);

// 		//     //check if the player is heading out of bounds on the x axis and undo the position change
// 		//     if (self.position.x - self.sprite.width() as i32 / 2) < -(SCREEN_WIDTH as i32 / 2) || (self.position.x + self.sprite.width() as i32 / 2) > SCREEN_WIDTH as i32 / 2{
// 		//         self.position = self.position.offset(-offset_x as i32, 0);
// 		//     }

// 		//     //check if the player is heading out of bounds on the y axis and undo the position change
// 		//     if (self.position.y - self.sprite.height() as i32 / 2) < -(SCREEN_HEIGHT as i32 / 2) || (self.position.y + self.sprite.height() as i32 /2) > SCREEN_HEIGHT as i32 / 2{
// 		//         self.position = self.position.offset(0,-offset_y as i32);
// 		//     }

// 		// }

// 	}
// 	//gets x and y components of a vector
// 	fn get_components(vector: Vector) -> (f64, f64){

// 	    let result_x = vector.magnitude * vector.direction.to_radians().cos();
// 	    let result_y = vector.magnitude * vector.direction.to_radians().sin();

// 	    (result_x, result_y)

// 	}

// 	fn construct_vector_components(x_comp: f64, y_comp: f64) -> Vector{

// 		Vector{

// 			magnitude: x_comp.powi(2) + y_comp.powi(2),
// 			direction: (y_comp/x_comp).atan().to_degrees(),

// 		}

// 	}

// 	//Transforms the x and y velocty vectors into coordinates to offset
// 	fn transform_vector(velocity_x: Vector, velocity_y: Vector, heading: f64) -> (f64, f64){

// 	    //vectors transformed to match the unit circle
// 	    let transformed_x = Vector{

// 	        magnitude: velocity_x.magnitude,
// 	        direction: -heading + velocity_x.direction,

// 	    };
// 	    let transformed_y = Vector{

// 	        magnitude: velocity_y.magnitude,
// 	        direction: -heading + velocity_y.direction,

// 	    };

// 	    let offset_x = get_components(transformed_x).0 + get_components(transformed_y).0;
// 	    let offset_y = get_components(transformed_x).1 + get_components(transformed_y).1;

// 	    (offset_x, -offset_y)

// 	}

// 	fn add_vectors(a: Vector, b: Vector) -> Vector{

// 		let a_comp = get_components(a);
// 		let b_comp = get_components(b);

// 		construct_vector_components(a_comp.0 + b_comp.0, a_comp.1 + b_comp.1)

// 	}

// 	//normalizes heading to wrap around between -180.0 and 180.0 in order to prevent heading from getting too large and causing floating point math errors
// 	fn normalize_heading(heading: f64) -> f64{

// 		if heading >= 180.0{return heading - 360.0}
// 		else if heading <= -180.0 {return heading + 360.0}
// 		else {return heading}

// 	}

// 	//searches the input stack and removes the specified input
// 	pub fn remove_input(input_stack: &mut Vec<Input>, remove: &Input){

// 		for i in 0..input_stack.len(){

// 			if input_stack[i] == *remove{

// 				input_stack.remove(i);
// 				break;

// 			}

// 		}

// 	}

// }
