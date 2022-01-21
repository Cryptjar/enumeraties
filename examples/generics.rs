// This file shows a how generic functions can utilize the `EnumProps` trait.

#![allow(dead_code)] // this is just an example

use enumeraties::props;
use enumeraties::EnumProp;

// The properties that we care about
struct ShapeDef {
	name: &'static str,
	vertices: u32,
}

// A generic function that works for any enum that has the `ShapeDef` property
fn print_shape<E>(e: E)
where
	E: EnumProp<ShapeDef>,
{
	println!(
		"A {} has {} vertices",
		e.property().name,
		e.property().vertices
	)
}

// One enum that will get the props
enum PlanarShape {
	Triangle,
	Square,
	Hexagon,
}
// Defining the properties on the `PlanarShape` enum
props! {
	impl Deref for PlanarShape as const ShapeDef {
		Self::Triangle => {
			name: "Triangle",
			vertices: 3,
		}
		Self::Square => {
			name: "Square",
			vertices: 4,
		}
		Self::Hexagon => {
			name: "Hexagon",
			vertices: 6,
		}
	}
}

// Another enum that will get the same props
enum Solid {
	Tetrahedron,
	Cube,
	Icosahedron,
}
// Defining the properties on the `Solid` enum
props! {
	impl Deref for Solid as const ShapeDef {
		Self::Tetrahedron => {
			name: "Tetrahedron",
			vertices: 4,
		}
		Self::Cube => {
			name: "Cube",
			vertices: 8,
		}
		Self::Icosahedron => {
			name: "Icosahedron",
			vertices: 12,
		}
	}
}


// Accessing the properties via the enum variants
pub fn main() {
	// Works with `PlanarShape`s
	print_shape(PlanarShape::Triangle);
	// And works with `Solid`s
	print_shape(Solid::Icosahedron);
}
