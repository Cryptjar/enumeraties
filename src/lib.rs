#![forbid(unsafe_code)]
#![cfg_attr(feature = "bench", feature(test))]
//! This crate provides a macro to add static, const, or lazy-initialized
//! properties to enum variants.
//!
//! This is a variation on the
//! [`enum_properties`](https://github.com/cofinite/enum_properties) crate.
//! It extends it and allows additionally to define multiple property structs
//! onto the same enum and adds support for `static` (instead of `const`)
//! properties as well as lazily initialized properties. However, this
//! additional features set comes with a syntax that is a bit more verbose,
//! hence, if you just need a single const-initialized property, you might find
//! `enum_properties` more concise. Nevertheless, you can also combine this
//! crate with `enum_properties` using the best of both as shown in the
//! [enum_props_combo example](https://github.com/Cryptjar/enumeraties/blob/master/examples/enum_props_combo.rs).
//!
//! See the [`props`](crate::props) macro for more details.
//!
//! # Example
//!
//! ```
//! use enumeraties::props;
//!
//! // A property struct
//! struct Prop { name: &'static str }
//!
//! // An enum
//! enum Foo {A}
//!
//! // Defining `Prop` on `Foo` via Deref
//! props! {
//!     impl Deref for Foo as const Prop {
//!         Self::A => {
//!             name: "Foo",
//!         }
//!     }
//! }
//!
//! // Accessing the property on `Foo`
//! assert_eq!(Foo::A.name, "Foo");
//! ```



/// The trait that is implemented through [`props`] macro.
///
/// This trait allows to write generic code that uses arbitrary enums that
/// happen to have specific properties defined on them.
///
/// # Example
///
/// ```
/// use enumeraties::props;
/// use enumeraties::EnumProp;
///
/// struct Prop {
///     name: &'static str,
/// }
///
/// // A generic function that works for any enum that has the `Prop` property
/// fn to_name<E>(e: E) -> &'static str
/// where
///     E: EnumProp<Prop>,
/// {
///     // The `Prop` struct can be accessed via the `property` method
///     e.property().name
/// }
///
/// // One enum that will get the props
/// enum Foo {
///     A,
///     B,
/// }
/// props! {
///     impl Deref for Foo as const Prop {
///         Self::A => {
///             name: "Foo",
///         }
///         Self::B => {
///             name: "Foobar",
///         }
///     }
/// }
///
/// // Another enum that will get the same props
/// enum Bar {
///     C,
///     D,
/// }
/// props! {
///     impl Deref for Bar as const Prop {
///         Self::C => {
///             name: "Bar",
///         }
///         Self::D => {
///             name: "Barfoo",
///         }
///     }
/// }
///
/// // Both enum can be used to call that function
/// assert_eq!(to_name(Foo::A), "Foo");
/// assert_eq!(to_name(Bar::C), "Bar");
/// ```
///
pub trait EnumProp<Prop> {
	fn property(&self) -> &'static Prop;
}

// For the macro
#[doc(hidden)]
pub use core::ops::Deref;

// Could still be feature gated
#[doc(hidden)]
pub use lazy_static; // 1.4.0

// The public front-end macro

/// Adds a property onto an enum
///
/// # Const, Static, Lazy
///
/// This macro allows implement properties in three different ways:
/// * as `const`, a constant
/// * as `static`, a global variable
/// * as `lazy`, a lazily initialized static
///
/// `const` and `static` are very similar, but have subtle difference:
/// the property type put into a `static` must implement `Send`. However,
/// with a `static` it is guaranteed that for each variant there is exactly one
/// unique property value and thus a unique reference address.
/// With `const` the compiler is allowed to merge properties (if they are equal)
/// or to inline and instantiated the same logical property multiple times,
/// i.e. the same logical property might be accessed via different reference
/// addresses.
/// In the very most cases, the actual reference address should not be of any
/// concern and thus it is recommended to use `const` over `static`.
///
/// One notable use-case for `static` is when the property contains interior
/// mutability.
/// In these cases, `const` shouldn't even compile.
///
/// `lazy`, on the other hand, is quite different from the `const` and `static`.
/// While `const` and `static` require constant initialized values computed at
/// compile-time, `lazy` allows to evaluate the value lazily at runtime,
/// instead. However, this feature incurs some overhead at each access, because
/// it must be checked that the value was indeed already initialized.
/// And of course, the first access to a `lazy` value, will incur the additional
/// delay to initialize the value.
///
///
/// # Syntax
///
/// This macro comes with essentially three different syntaxes: to implement
/// `Deref` (for the primary property), add an inherent access method
/// (for secondary properties), or just implementing `EnumProp` onto it (e.g.,
/// if only used by generic code).
///
/// ## Implementing [Deref](core::ops::Deref)
///
/// Syntax:
///
/// ```text
/// impl Deref for <ENUM> as (const|static|lazy) <PROPERTY> {
///     <VARIANT> => {
///         <FIELD> : <VALUE>,
///         ...
///     },
///     ...
/// }
/// ```
///
/// Example:
///
/// ```
/// # use enumeraties::props;
/// struct Prop { name: &'static str }
/// enum Foo {A}
/// props! {
///     impl Deref for Foo as const Prop {
///         Self::A => {
///             name: "Foo",
///         }
///     }
/// }
/// // Direct access due to deref
/// assert_eq!(Foo::A.name, "Foo");
/// ```
///
///
/// ## Implementing an inherent method
///
/// Syntax:
///
/// ```text
/// impl <ENUM> : <VIS> fn <FN_NAME> as (const|static|lazy) <PROPERTY> {
///     <VARIANT> => {
///         <FIELD> : <VALUE>,
///         ...
///     },
///     ...
/// }
/// ```
///
/// Example:
///
/// ```
/// # use enumeraties::props;
/// struct Prop { name: &'static str }
/// enum Foo {A}
/// props! {
///     // Of course, an arbitrary function names can be used instead of `getter`
///     impl Foo : fn getter as const Prop {
///         Self::A => {
///             name: "Foo",
///         }
///     }
/// }
/// // Access via inherent method
/// assert_eq!(Foo::A.getter().name, "Foo");
/// ```
///
/// ## Implementing only `EnumProp`
///
/// Syntax:
///
/// ```text
/// impl EnumProp for <ENUM> as (const|static|lazy) <PROPERTY> {
///     <VARIANT> => {
///         <FIELD> : <VALUE>,
///         ...
///     },
///     ...
/// }
/// ```
///
/// Example:
///
/// ```
/// # use enumeraties::props;
/// struct Prop { name: &'static str }
/// enum Foo {A}
/// props! {
///     impl EnumProp for Foo as const Prop {
///         Self::A => {
///             name: "Foo",
///         }
///     }
/// }
/// // Access via universal function call
/// use enumeraties::EnumProp;
/// assert_eq!(EnumProp::<Prop>::property(&Foo::A).name, "Foo");
/// ```
///
#[macro_export]
macro_rules! props {
	(
		// A lazy/const impl that will be promoted to `Deref` (also impls `EnumProp`)
		impl Deref for $enum_name:ty as $modifier:ident $prop_name:path { $($matching:tt)* }
	) => {
		// Add the EnumProp impl
		$crate::internal_props_impl_macro!{
			@EnumProp
			mod($modifier) ($prop_name) for $enum_name {
				$($matching)*
			}
		}

		// Add the deref forwarding
		impl $crate::Deref for $enum_name {
			type Target = $prop_name;
			fn deref(&self) -> &Self::Target {
				$crate::EnumProp::<$prop_name>::property(self)
			}
		}
	};
	(
		// The lazy/const impl via inherent method (also impls `EnumProp`)
		impl $enum_name:ty : $fn_vis:vis fn $fn_name:ident as $modifier:ident $prop_name:path { $($matching:tt)* }
	) => {
		// Add the EnumProp impl
		$crate::internal_props_impl_macro!{
			@EnumProp
			mod($modifier) ($prop_name) for $enum_name {
				$($matching)*
			}
		}

		// Add the inherent method forwarding
		impl $enum_name {
			$fn_vis fn $fn_name(&self) -> &'static $prop_name {
				$crate::EnumProp::<$prop_name>::property(self)
			}
		}
	};
	(
		// The lazy/const impl `EnumProp` only
		impl EnumProp for $enum_name:ty as $modifier:ident $prop_name:path { $($matching:tt)* }
	) => {
		// Add the EnumProp impl
		$crate::internal_props_impl_macro!{
			@EnumProp
			mod($modifier) ($prop_name) for $enum_name {
				$($matching)*
			}
		}
	};
}

// The internal marco impl, used by `props`, do not use, its API may change
// at any time
#[doc(hidden)]
#[macro_export]
macro_rules! internal_props_impl_macro {
	(
		// The enum prop impl, entry rule
		@EnumProp
		mod($modifier:ident) ($prop_name:path) for $enum_name:ty {
			$(
				// True match branches, could be simplified to `ident`, but then
				// one can on longer identify e.g. `Beta(42)` (maybe one shouldn't)
				$branch:pat => {
					$(
						$struct_fields:tt
					)*
				} $(,)?
			)*
		}
	) => {
		impl $crate::EnumProp<$prop_name> for $enum_name {
			fn property(&self) -> &'static $prop_name {
				#[deny(unreachable_patterns)] // Remember the `Self` prefix
				match self {
					$(
						$branch => {
							$crate::internal_props_impl_macro!(
								@Branch mod($modifier) $prop_name {
									$( $struct_fields )*
								}
							)
						},
					)*
				}
			}
		}
	};

	(
		// A single *const* prop value
		@Branch
		mod(const) $prop_name:path {
			$(
				$field:ident : $value:expr
			),* $(,)?
		}
	) => {{
		// A const reference, given that all `$value`s are const-init

		// Notice, having the explicit constant gives clearer error messages.

		// `BAR` is rather arbitrary here, maybe different name would be better
		const BAR : $prop_name = {
			$prop_name {
				$(
					$field : $value ,
				)*
			}
		};

		& BAR
	}};

	(
		// A single *static* prop value
		@Branch
		mod(static) $prop_name:path {
			$(
				$field:ident : $value:expr
			),* $(,)?
		}
	) => {{
		// A static reference given, that all `$value`s are const-init

		// `BAZ` is rather arbitrary here, maybe different name would be better
		static BAZ : $prop_name = {
			$prop_name {
				$(
					$field : $value ,
				)*
			}
		};

		& BAZ
	}};

	(
		// A single *const* prop value
		@Branch
		mod(lazy) $prop_name:path {
			$(
				$field:ident : $value:expr
			),* $(,)?
		}
	) => {{
		// A static reference via lazy_static.

		// `FOO` is rather arbitrary here, maybe different name would be better
		$crate::lazy_static::lazy_static!{
			static ref FOO: $prop_name = {
				$prop_name {
					$(
						$field : $value ,
					)*
				}
			};
		}

		&*FOO
	}};
}

// Some testing modules

mod benchs;
mod test_static;


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
