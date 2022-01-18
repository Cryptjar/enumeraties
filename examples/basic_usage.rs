#![allow(dead_code)] // this is just an example

use enumeraties::props;

// An enum that will gets some props
enum Shapes {
	Triangle,
	Square,
	Hexagon,
}

// The properties to add onto the enum
struct ShapeDef {
	name: &'static str,
	vertices: u32,
	internal_angle: f32, // in radian
}

// Defining the properties on the enum
props! {
	impl Deref for Shapes as const ShapeDef {
		Self::Triangle => {
			name: "Triangle",
			vertices: 3,
			internal_angle: core::f32::consts::PI / 3.,
		}
		Self::Square => {
			name: "Square",
			vertices: 4,
			internal_angle: core::f32::consts::PI / 4.,
		}
		Self::Hexagon => {
			name: "Hexagon",
			vertices: 6,
			internal_angle: core::f32::consts::PI / 6.,
		}
	}
}

// Accessing the properties via the enum variants
pub fn main() {
	println!(
		"A {} has {} vertices and an internal angle of {:.3} radian.",
		Shapes::Hexagon.name,           // gives "Hexagon"
		Shapes::Hexagon.vertices,       // gives 6
		Shapes::Hexagon.internal_angle  // gives π/6
	)
}
