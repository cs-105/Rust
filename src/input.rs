pub mod controller{

	extern crate sdl2;

	use sdl2::rect::{Rect, Point};

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Input{

		Up,
		Down,
		Left,
		Right,

	}

	//TODO: Move to separate file
	//Physics vector, mainly used for storing velocity
	#[derive(Debug, Clone, Copy)]
	pub struct Vector{

	    pub magnitude: f64,
	    pub direction: f64, 

	}

	//TODO: Move to separate file
	//Player Struct keeps track of data about the player avatar
	#[derive(Debug)]
	pub struct Player{

	    pub position: Point, //2-D Cartesian Point 
	    pub sprite: Rect, //Dimensions are used to select what to render from the spritesheet
	    pub speed: u32, //Rate at which the player sprite is moved
	    pub heading: f64, //Heading of the player

	}

	impl Player{



	}
	
	//TODO: Move to separate file
	//gets x and y components of a vector
	fn get_components(vector: Vector) -> (f64, f64){

	    let result_x = vector.magnitude * vector.direction.to_radians().cos();
	    let result_y = vector.magnitude * vector.direction.to_radians().sin();

	    (result_x, result_y)

	}

	//TODO: Move to separate file
	//Transforms the x and y velocty vectors into coordinates to offset
	pub fn transform_vector(velocity_x: Vector, velocity_y: Vector, heading: f64) -> (f64, f64){

	    //vectors transformed to match the unit circle
	    let transformed_x = Vector{

	        magnitude: velocity_x.magnitude,
	        direction: heading + 90.0 + velocity_x.direction,

	    };
	    let transformed_y = Vector{

	        magnitude: velocity_y.magnitude,
	        direction: heading + 90.0 + velocity_y.direction,

	    };

	    let offset_x = get_components(transformed_x).0 + get_components(transformed_y).0;
	    let offset_y = get_components(transformed_x).1 + get_components(transformed_y).1;

	    (offset_x, -offset_y)

	}

	pub fn remove_input(input_stack: &mut Vec<Input>, remove: &Input){

		for i in 0..input_stack.len(){

			if input_stack[i] == *remove{

				input_stack.remove(i);
				break;

			}

		}

	}

}