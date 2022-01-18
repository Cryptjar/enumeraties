// This file showcases how `enum_properties` can be combined with this crate

#![allow(dead_code)] // this is just an example

use enum_map::enum_map;
use enum_map::Enum;
use enum_map::EnumMap;
use enum_properties::enum_properties;
use enumeraties::props;

// Some basic const-initializable properties to add onto the enum
struct ShapeDef {
	name: &'static str,
	vertices: usize,
	internal_angle: f32,
}

// An enum wrapped in `enum_properties` to directly define `ShapeDef` onto it
enum_properties! {
	#[derive(Enum)]
	enum Shape: ShapeDef {
		Triangle {
			name: "Triangle",
			vertices: 3,
			internal_angle: core::f32::consts::PI / 3.,
		},
		Square {
			name: "Square",
			vertices: 4,
			internal_angle: core::f32::consts::PI / 4.,
		},
		Hexagon {
			name: "Hexagon",
			vertices: 6,
			internal_angle: core::f32::consts::PI / 6.,
		},
	}
}

// A non-const initializable property, this wouldn't work with `enum_properties`,
// however we can add it on top the `ShapeDef` property with this crate.
struct LazyProps {
	description: String,
	// assuming the same base line, as self/other
	area_ration_with: EnumMap<Shape, f32>,
}


// Defining the non-const properties lazily onto the enum and make it
// accessible via the inherent method `props`.
props! {
	impl Shape : fn props as lazy LazyProps {
		Self::Triangle => {
			// Laziness allows to run arbitrary init code
			description: [1,2,3].iter().map(ToString::to_string).collect(),
			area_ration_with: enum_map! {
				Shape::Triangle => 1.0,
				// Also notice that `sqrt` is as the time of writing not a `const fn`
				Shape::Square => 3_f32.sqrt() / 4_f32,
				Shape::Hexagon => 1.0 / 6.0,
			},
		}
		Self::Square => {
			description: String::new(),
			area_ration_with: enum_map! {
				Shape::Triangle => 1.0,
				Shape::Square => 4_f32 / 3_f32.sqrt(),
				Shape::Hexagon => 2_f32 / (3_f32 * 3_f32.sqrt()),
			},
		}
		Self::Hexagon => {
			description: {
				let mut buf = String::new();
				buf .push_str("stuff");
				buf
			},
			area_ration_with: enum_map! {
				Shape::Triangle => 6.0,
				Shape::Square => 3_f32 * 3_f32.sqrt() / 2_f32,
				Shape::Hexagon => 1.0,
			},
		}
	}
}

// Accessing the properties via the enum variants
pub fn main() {
	println!(
		"A {} has {} vertices and an internal angle of {:.3} radian.",
		Shape::Hexagon.name,           // gives "Hexagon"
		Shape::Hexagon.vertices,       // gives 6
		Shape::Hexagon.internal_angle  // gives π/6
	);
	println!(
		"A Hexagon has {:.3} more area than a Square.",
		// gives 3*sqr(3)/2
		Shape::Hexagon.props().area_ration_with[Shape::Square]
	);
}
