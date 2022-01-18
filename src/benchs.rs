// This is only a testing module, i.e. it is `cfg(test)` and needs the `bench`
// crate feature.
#![cfg(test)]
#![cfg(feature = "bench")]

extern crate test;
use test::Bencher;

use super::EnumProp;


// A simple integer property
struct IntPropConst {
	int: u32,
}
struct IntPropLazy {
	int: u32,
}

fn test_1000<E: Copy, P: 'static>(e: E, mut f: impl FnMut(&P) -> u32) -> u32
where
	E: EnumProp<P>,
{
	let mut sum = 0;
	for _ in 0..1000 {
		let s = test::black_box(e);
		sum += f(s.property())
	}
	sum
}



// A simple singleton enum
#[derive(Copy, Clone)]
enum Singleton {
	Foo,
}

props! {
	impl EnumProp for Singleton as const IntPropConst {
		Singleton::Foo => {
			int: 42
		}
	}
}
#[bench]
fn singelton_1000_const_access(b: &mut Bencher) {
	b.iter(|| test_1000(Singleton::Foo, |p: &IntPropConst| p.int));
}


props! {
	impl EnumProp for Singleton as lazy IntPropLazy {
		Singleton::Foo => {
			int: 42
		}
	}
}
#[bench]
pub fn singelton_1000_lazy_access(b: &mut Bencher) {
	b.iter(|| test_1000(Singleton::Foo, |p: &IntPropLazy| p.int));
}



// A quad variant enum
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Quad {
	A,
	B,
	C,
	D,
}

props! {
	impl EnumProp for Quad as const IntPropConst {
		Quad::A => {
			int: 3
		}
		Quad::B => {
			int: 5
		}
		Quad::C => {
			int: 7
		}
		Quad::D => {
			int: 11
		}
	}
}
#[bench]
pub fn quad_1000_const_access(b: &mut Bencher) {
	b.iter(|| test_1000(Quad::C, |p: &IntPropConst| p.int));
}

props! {
	impl EnumProp for Quad as lazy IntPropLazy {
		Quad::A => {
			int: 3
		}
		Quad::B => {
			int: 5
		}
		Quad::C => {
			int: 7
		}
		Quad::D => {
			int: 11
		}
	}
}
#[bench]
pub fn quad_1000_lazy_access(b: &mut Bencher) {
	b.iter(|| test_1000(Quad::C, |p: &IntPropLazy| p.int));
}
