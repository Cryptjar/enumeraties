// This file shows the usage of enums with variant data

#![allow(dead_code)] // this is just an example

use enumeraties::props;

// An enum that will gets some props
enum Foo {
	Alpha,
	Beta(u32),
	Gamma { float: f32 },
}

// The properties to add onto the enum
struct Bar {
	name: &'static str,
}

// Defining the properties on the enum
props! {
	impl Deref for Foo as const Bar {
		Self::Alpha => {
			name: "Alpha",
		}
		Self::Beta(_) => {
			name: "Beta",
		}
		Self::Gamma{..} => {
			name: "Gamma",
		}
	}
}

// Accessing the properties via the enum variants
pub fn main() {
	println!(
		"{} - {} - {}",
		Foo::Alpha.name,
		Foo::Beta(12).name,
		Foo::Gamma {
			float: 1.23
		}
		.name,
	)
}
