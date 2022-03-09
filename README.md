Enumeraties
===========

> Enumeration types with Properties

<!-- cargo-sync-readme start -->

This crate provides a macro to add static, const, or lazy-initialized
properties to enum variants.

This is a variation on the
[`enum_properties`](https://github.com/cofinite/enum_properties) crate.
It extends it and allows additionally to define multiple property structs
onto the same enum and adds support for `static` (instead of `const`)
properties as well as lazily initialized properties. However, this
additional features set comes with a syntax that is a bit more verbose,
hence, if you just need a single const-initialized property, you might find
`enum_properties` more concise. Nevertheless, you can also combine this
crate with `enum_properties` using the best of both as shown in the
[enum_props_combo](examples/enum_props_combo.rs) example.

See the [`props`](https://docs.rs/enumeraties/latest/enumeraties/macro.props.html) macro for more details.

# Example

```rust
use enumeraties::props;

// A property struct
struct Prop { name: &'static str }

// An enum
enum Foo {A}

// Defining `Prop` on `Foo` via Deref
props! {
    impl Deref for Foo as const Prop {
        Self::A => {
            name: "Foo",
        }
    }
}

// Accessing the property on `Foo`
assert_eq!(Foo::A.name, "Foo");
```

<!-- cargo-sync-readme end -->
