#![forbid(unsafe_code)]


// *The* trait
pub trait EnumProp<Prop> {
	fn property(&self) -> &'static Prop;
}

// For the macro
#[doc(hidden)]
pub use core::ops::Deref;

// Could still be feature gated
#[doc(hidden)]
pub use lazy_static; // 1.4.0

/*
props! {
	impl Deref for Foo as const PropsConst {
		...
	}
}

props! {
	impl Foo pub fn inherent as const PropsConst {
		...
	}
}
*/

// The public front-end macro

/// Adds a property onto an enum
///
/// Basic syntax using [Deref](core::ops::Deref):
///
/// ```text
/// impl Deref for <ENUM> as (lazy|const) <PROPERTY> {
///     <VARIANT> => {
///         <FIELD> : <VALUE>,
///         ...
///     },
///     ...
/// }
/// ```
///
/// Basic syntax for inherent method access:
///
/// ```text
/// impl <ENUM> : <VIS> fn <FN_NAME> as (lazy|const) <PROPERTY> {
///     <VARIANT> => {
///         <FIELD> : <VALUE>,
///         ...
///     },
///     ...
/// }
/// ```
///
/// Basic syntax for only the trait impl:
///
/// ```text
/// impl EnumProp for <ENUM> as (lazy|const) <PROPERTY> {
///     <VARIANT> => {
///         <FIELD> : <VALUE>,
///         ...
///     },
///     ...
/// }
/// ```
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

		// Just some fancy header, we could add `Deref` somewhere to make clear
		// that that's what we actually implement
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

		// Just some fancy header, we could add `Deref` somewhere to make clear
		// that that's what we actually implement
		mod(const) $prop_name:path {
			$(
				$field:ident : $value:expr
			),* $(,)?
		}
	) => {{
		// A static reference given, that all `$value`s are const-init
		// Notice the difference between `static` and `const` here:
		// Both need const-init data, but only `static` requires `Send`, i.e.
		// `const` has less requirements, both work here.
		//
		// However, with `const` it is possible that two invocations of the
		// surrounding function return different references (i.e. different
		// addresses) because const gets inlined.
		//
		// If this subtle difference is important, it is conceivable to present
		// both a `const` and a `static` variant.
		//
		// So, it just should be pointed out somewhere (in the docs).

		// Notice, having the explicit constant/static gives clearer error messages.

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
		// A single *const* prop value
		@Branch

		// Just some fancy header, we could add `Deref` somewhere to make clear
		// that that's what we actually implement
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


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
