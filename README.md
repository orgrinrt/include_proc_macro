# `include_proc_macro`

[![Crates.io](https://img.shields.io/crates/v/include_proc_macro)](https://crates.io/crates/include_proc_macro) [![docs.rs](https://img.shields.io/docsrs/include_proc_macro)](https://docs.rs/include_proc_macro)

> A `macro_rules!` for declaring a crate's procedural macros in one list, with the implementations wherever they live.

A procedural macro has to be a public function at the root of its proc-macro crate, with the attribute for its kind and the exact signature that kind takes, but the implementation doesn't have to be there. Once a crate has more than a few, the root turns into a stack of delegating stubs, each a `mod` line, an attribute, a signature and a body forwarding to the real function. This crate writes those stubs: a declaration says what kind of macro it is, what it's called and where the implementation is, and the delegation is generated from that, `mod` declaration included where one is needed.

It is `macro_rules!` only and has no dependencies, so everything happens during expansion and nothing reaches the compiled artifact. Crates using your macros depend on your proc-macro crate and never on this one.

## Usage

```bash
cargo add include_proc_macro
```

The three kinds each have a macro, `proc_macro!`, `attr_macro!` and `derive_macro!`, and `macros!` declares any number of them in one block. Each takes a name on the left and an implementation path on the right, and the derive one takes its name in parentheses since a derive may declare helper attributes there.

```rust,ignore
include_proc_macro::proc_macro!(sql -> parsing::parse_sql);
include_proc_macro::attr_macro!(instrument -> tracing_impl::instrument);
include_proc_macro::derive_macro!((Validate, attributes(required)) -> derives::validate);
```

These blocks are marked `ignore` because the output carries `#[proc_macro]`, which rustc only accepts in a crate whose manifest says `proc-macro = true`, and a doctest is compiled as an ordinary crate. They are checked in `arm_matrix_test/` instead, with the refusals in a compile-fail suite beside it.

The implementation path takes the same forms for all three kinds. A bare `f` or `use f` is a function already in scope, `m::f` or `mod m::f` declares `mod m` and calls `m::f`, `use m::f` says `m` is already declared, and a longer path like `a::b::c::f` declares `mod a`. A path starting with `crate::` or `self::` is already reachable so nothing is declared, and a file can be named as `"path/to/file.rs"::f` relative to the invocation, or `@"path/from/crate/root.rs"::f` relative to the crate root. In the list form the macro name can be left out for functions and attributes and then comes from the last segment of the path, except for a bare name, where the generated item would shadow the function it calls.

There is no `super::` form, since the root of a crate has no parent.

## Status

Complete for what it sets out to do, and the 2.x line has only added to it. The floor is rust 1.56. The `no_std` and `no_alloc` features only name what is already true of this crate, and they say nothing about the crate using it, since a proc-macro crate runs on the host and can't be `no_std`.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/include_proc_macro/blob/main/LICENSE)
