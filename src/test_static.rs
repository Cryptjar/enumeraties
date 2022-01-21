// This file test attributes of static and const properties
#![cfg(any(test, doctest))]
#![allow(dead_code)]

// Notice, these "examples" are done here as tests instead of examples, because
// they contain non recommended usage-patterns.
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;

#[test]
fn foo_const_refs() {
	struct FooStatic {
		foo: AtomicU8,
	}

	enum Foo {
		A,
	}

	props! {
		impl Deref for Foo as static FooStatic {
			Self::A => {
				foo: AtomicU8::new(42),
			}
		}
	}

	let old: u8 = Foo::A.foo.load(Ordering::SeqCst);
	assert_eq!(old, 42);

	// Mutable statics can be changed, and its preserved globally

	Foo::A.foo.store(13, Ordering::SeqCst);

	let new: u8 = Foo::A.foo.load(Ordering::SeqCst);
	assert_eq!(new, 13);
}



// Notice that AtomicU8, or apparently any mutable type dose not work with const

/// ```compile_fail,E0515
/// use enumeraties::props;
/// use core::sync::atomic::AtomicU8;
///
/// struct Props {
///     foo: AtomicU8,
/// }
///
/// enum Foo {
///     A,
/// }
///
/// props! {
///     impl Deref for Foo as const Props {
///         Self::A => {
///             foo: AtomicU8::new(42),
///         }
///     }
/// }
/// ```
struct NoAtomicWithConst;
