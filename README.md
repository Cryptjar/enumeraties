Enumeraties
===========

> Enumeration types with Properties

This crate provides a macro for declaring static, const, or lazy properties on enum variants.

This crate is a variation on the [`enum_properties`](https://github.com/cofinite/enum_properties) crate. It extends it and allows additionally to define multiple property structs onto the same enum and adds support for `static` (instead of `const`) properties as well as lazily initialized properties. However, this additional features set comes with a syntax that is a bit more verbose, hence, if you just need a single const-initialized property, you might find `enum_properties` more concise. Nevertheless, you can also combine this crate with `enum_properties` using the best of both as shown in the [enum_props_combo](examples/enum_props_combo.rs) example.
