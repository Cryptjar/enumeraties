Enumeraties
===========

> Enumeration types with Properties

This crate provides a macro for declaring static, const, or lazy properties on enum variants.

This crate is variation on the [`enum_properties`](https://github.com/cofinite/enum_properties) crate, which essentially allows to define multiple property structs onto the same enum as well as `static` and lazily initialized `static` properties. Tho, the syntax is a bit more verbose, thus if you need just a single const-initialized property you may as well use `enum_properties` instead. However, you can also combine this crate with `enum_properties` using the best of both as shown in the [enum_props_combo](examples/enum_props_combo.rs) example.
