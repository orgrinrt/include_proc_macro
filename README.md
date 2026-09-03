# `include_proc_macro`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/stargazers)
[![Crates.io](https://img.shields.io/crates/v/include_proc_macro)](https://crates.io/crates/include_proc_macro)
[![docs.rs](https://img.shields.io/docsrs/include_proc_macro)](https://docs.rs/include_proc_macro)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/issues)
![License](https://img.shields.io/github/license/orgrinrt/include_proc_macro?color=%23009689)

> A `macro_rules!` for declaring a crate's procedural macros in one list, with the implementations wherever they happen to live.

</div>

A procedural macro has to be a public function at the root of its proc-macro crate, carrying the
attribute for its kind and the exact signature that kind takes, but the implementation need not be
there, and in a crate with more than a handful of macros it probably shouldn't be either. What the
root then ends up holding is a stack of delegating stubs, each one a `mod` line, an attribute, a
signature and a one-line body forwarding to the real function, repeated per macro with only the
names changing between them.

This crate writes those stubs. A declaration says what kind of macro it is, what it's called and
where the implementation is, and the delegation gets generated from that, `mod` declaration
included where one is needed. The implementations stay ordinary functions in ordinary modules, or
in files outside the module tree altogether, since there's a path form for that too.

It's `macro_rules!` only, with no dependencies of its own, so all of it happens during expansion and
nothing new reaches the compiled artifact. Crates using the macros aren't affected either way, as
they depend on the proc-macro crate and never on this one.

## Usage

```bash
cargo add include_proc_macro
```

The three kinds each have a macro of their own, `proc_macro!`, `attr_macro!` and `derive_macro!`,
and `macros!` declares any number of them in one block. Each takes a name on the left and an
implementation path on the right, and the derive one takes its name in parentheses, since a derive
may also declare helper attributes there:

```rust,ignore
include_proc_macro::proc_macro!(sql -> parsing::parse_sql);
include_proc_macro::attr_macro!(instrument -> tracing_impl::instrument);
include_proc_macro::derive_macro!((Validate, attributes(required)) -> derives::validate);
```

Do note that the rust blocks declaring a macro are marked `ignore` here. The output carries
`#[proc_macro]`, which rustc accepts only in a crate whose manifest says `proc-macro = true`, and a
doctest is compiled as an ordinary crate, so these can't run as doctests whatever they contain.
They're checked in the repository's `arm_matrix_test/` crate instead, every path form against every
kind and every way of naming one, with the refusals in a compile-fail suite beside it.

The implementation path takes the same forms for all three kinds, in the list form too, as one
shared piece of the crate reads them whichever kind is being declared:

| Form | Means |
|---|---|
| `f` | `f` is already in scope |
| `use f` | the same, said explicitly |
| `m::f` | declare `mod m`, call `m::f` |
| `mod m::f` | the same, said explicitly |
| `use m::f` | `m` is already declared |
| `a::b::c::f` | declare `mod a`, call `a::b::c::f` |
| `use a::b::c::f` | `a` is already declared |
| `crate::m::f`, `self::m::f` | already reachable, so nothing is declared |
| `"path/to/file.rs"::f` | a file, named relative to the invocation |
| `@"path/from/crate/root.rs"::f` | a file, named relative to the crate root |

The `mod` form is what a bare path means anyway, so it's there for readability, and `use` is for a
module some earlier line already declared, since declaring it twice is an error. In the list form
the macro name can be left out for `function` and `attribute`, in which case it comes from the last
segment of the path, though not for a path that is a bare name, as the generated item would then
take the same name in the same scope and shadow the function it means to call. A derive always
names itself, the name being what the deriving type writes.

## Example

Here's a proc-macro crate root with all three kinds in one block, and most of the path forms in
use:

```rust,ignore
use include_proc_macro::macros;

macros!(
    // a function-like macro; declares `mod parsing` and forwards to it
    function(sql) -> parsing::parse_sql,
    // no name given, so it's taken from the last segment: `tokenize`
    function -> parsing::tokenize,
    // `parsing` was declared above, so `use` it; a second `mod parsing` would be an error
    function(lex) -> use parsing::lex,

    // an attribute macro; the implementation takes two token streams
    attribute(instrument) -> tracing_impl::instrument,
    // modules nest to any depth
    attribute(cached) -> caching::memo::store::apply,

    // a derive; the name in parentheses is what deriving types write
    derive(Builder) -> derives::builder,
    // helper attributes go beside it, any number of them
    derive(Validate, attributes(required, length, range)) -> derives::validate,

    // a file outside the module tree, relative to this one
    function(greet) -> "impls/hello.rs"::greet,
    // or relative to the crate root, with a leading `@`
    derive(Display) -> @"tests/fixtures/display.rs"::display,
);
```

The implementations on the other side are plain functions with the token stream signature of their
kind and no attribute on them, the attribute being legal at the crate root only:

```rust
# extern crate proc_macro;
// parsing.rs
use proc_macro::TokenStream;

pub fn parse_sql(input: TokenStream) -> TokenStream {
    // the actual work goes here
    input
}

pub fn tokenize(input: TokenStream) -> TokenStream {
    input
}

pub fn lex(input: TokenStream) -> TokenStream {
    input
}
```

That's nine declarations, and what they stand in for is below, though a crate with this many
macros in one place is probably not the usual case anyway.

<details>
<summary>The same nine declarations, by hand</summary>

```rust,ignore
mod parsing;
mod tracing_impl;
mod caching;
mod derives;

#[proc_macro]
pub fn sql(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parsing::parse_sql(input)
}
#[proc_macro]
pub fn tokenize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parsing::tokenize(input)
}
#[proc_macro]
pub fn lex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parsing::lex(input)
}
#[proc_macro_attribute]
pub fn instrument(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    tracing_impl::instrument(attr, item)
}
#[proc_macro_attribute]
pub fn cached(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    caching::memo::store::apply(attr, item)
}
#[allow(non_snake_case)]
#[proc_macro_derive(Builder)]
pub fn Builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derives::builder(input)
}
#[allow(non_snake_case)]
#[proc_macro_derive(Validate, attributes(required, length, range))]
pub fn Validate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derives::validate(input)
}
#[proc_macro]
pub fn greet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[path = "impls/hello.rs"]
    mod __inner;
    __inner::greet(input)
}
#[allow(non_snake_case)]
#[proc_macro_derive(Display)]
pub fn Display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mod __inner {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/display.rs"));
    }
    __inner::display(input)
}
```

</details>

There are three runnable examples under `integration_test/examples/` in the repository as well, one
with a single function-like macro, one reaching all three kinds from one block, and one walking
every path form in the table with a line of output per form:

```text
cargo run -p integration_test --example one_function_macro
cargo run -p integration_test --example all_three_kinds
cargo run -p integration_test --example every_path_form
```

`cargo test` runs them too and checks what each one prints.

## Motivation

A proc macro has to be a public function at the root of its crate, which either leaves the root
module gigantic and hard to find anything in, or, with the implementations moved out into modules
the way this crate does underneath, leaves it full of the same delegation written over and over,
attributes and signatures spelled out each time. It's tedious more than anything, and in a larger
proc-macro crate with several kinds of macro in it the root stops saying anything useful about what
the crate exports.

So the point here is a declaration list that reads as an index of the crate, one line per macro
saying its kind, its name and where the code is, with the delegation generated instead of typed.
The file forms are a side benefit of the same mechanism, for keeping implementations apart from the
module tree, having the macro tests somewhere sensible, or whatever else calls for a path.

As for the cost, it's a compile-time dependency for the proc-macro crate and nothing beyond that,
and it pulls nothing in itself. `macros!` recurses once per declaration rather than once per token,
and a hundred declarations in one invocation is what the test suite asserts under rustc's default
recursion limit of 128. Past that, splitting the list or raising `#![recursion_limit]` both work.

## Extras

### Status

Complete for what it sets out to do, and the 2.x line has only added to it, path forms and the
two features, with nothing taken away. The floor is rust 1.56, which is what edition 2021 needs.

### Cargo features

| Feature | Default | Effect |
|---|---|---|
| `no_std` | off | Adds `#![no_std]` to this crate, which is the attribute and nothing more, there being no code in here for it to affect. |
| `no_alloc` | off | Implies `no_std`. States what is already the case, since nothing here allocates. |

Both exist so a workspace turning them on everywhere can name them without the build failing on an
unknown feature. Neither changes what the macros expand to, and `tests/feature_matrix.rs` builds
under each selection and asserts they still expand.

### Limitations

What the macros expand into is a `#[proc_macro]` entry point, and a proc-macro crate can't be
`no_std` whatever this one declares, since it runs on the host inside the compiler. So the features
above are about this crate, and say nothing about the crate using it.

There's no `super::` path form, since a procedural macro item has to sit at the crate root and the
root has no parent. An absolute path works in the plain literal form, as it goes straight into
`#[path]`, though what that means for portability is outside this crate's scope.

## Support

Feel free to contribute! If unsure about wasting work, the best practice is to throw in an issue describing what you'd do, and only then commit to writing a big PR, because chances are, it might not be something that belongs here. However, forks are always a valid choice and we'd encourage everyone to experiment and have their own takes on this. When doing this, do mind the license(s) though!

A new path form or a new refusal wants a row in the arm matrix and a case in the compile-fail suite beside it.

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/include_proc_macro/blob/main/LICENSE)
