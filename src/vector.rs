pub mod vector{

	use std::ops;

	pub struct Vector{

		magnitude: f64, //magnitude of vector, unitless
		direction: f64, //direction of vector in degrees
		x: f64, //x component
		y: f64, //y component

	}

	//overloads
	impl PartialEq for Vector{

		fn eq(&self, other: &Self) -> bool{

			self.magnitude == other.magnitude && self.direction == other.direction

		}

	}

	impl eq for Vector{}

	impl ops::Add<Vector> for Vector{

		fn add(&mut self, _rhs: Vector) -> Vector{

		let a_comp = get_components(self);
		let b_comp = get_components(_rhs);

		construct_vector_components(a_comp.0 + b_comp.0, a_comp.1 + b_comp.1)

		}

	}

	impl Vector{

		//getters
		pub fn get_magnitude(&mut self) -> f64{self.magnitude}
		pub fn get_direction(&mut self) -> f64{self.direction}
		pub fn get_x(&mut self) -> f64{self.x}
		pub fn get_y(&mut self) -> f64{self.y}
		pub fn get_components(&mut self) -> (f64, f64){(self.x, self.y)}

		//setters
		pub fn set_magnitude(&mut self, new_magnitude: f64){self.magnitude = new_magnitude}
		pub fn set_direction(&mut self, new_direction: f64){self.direction = new_direction}
		pub fn set_x(&mut self, new_x: f64){self.x = new_x}
		pub fn set_y(&mut self, new_y: f64){self.y = new_y}
		pub fn set_components(&mut self, new_x: f64, new_y: f64){

			self.x = new_x;
			self.y = new_y;

		}

		//constructors
		pub fn create_vector(magnitude: f64, direction: f64) -> Vector{

			Vector{

				magnitude,
				direction,
				x: magnitude * direction.to_radians().cos(),
				y: magnitude * direction.to_radians().sin(),

			}


		}

		pub fn construct_vector_components(x_comp: f64, y_comp: f64) -> Vector{

			Vector{

				magnitude: x_comp.powi(2) + y_comp.powi(2),
				direction: (y_comp/x_comp).atan().to_degrees(),
				x: x_comp,
				y: y_comp,

			}

		}

		//functions

	}

}