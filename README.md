include_proc_macro
============

<div style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/stargazers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/include_proc_macro)](https://crates.io/crates/include_proc_macro)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/issues)
[![Current Version](https://img.shields.io/badge/version-2.1.0-blue.svg)](https://github.com/orgrinrt/include_proc_macro)

> A convenient macro for working with multiple procedural macros in one crate, and to import them from any arbitrary paths. Reduces boilerplate and repetition, and improves readability.

</div>

## The problem

Rust requires every procedural macro to be a public function at the root of a proc-macro
crate, carrying the right attribute and the exact signature for its kind. The
implementations do not have to live there, and in any crate with more than a handful of
macros they should not, so the root fills up with delegating stubs: a `mod` line, an
attribute, a signature, and a one-line body that forwards to the real code.

None of that carries information. It is the same four lines each time, and the only part
that varies is which function is being forwarded to.

This crate writes those stubs. You say what kind of macro it is, what it is called, and
where the implementation lives, and the delegation is generated. There is nothing at
runtime and no dependencies; it is one `macro_rules!` file that expands to the code you
would otherwise type.

## Usage

```rust,ignore
use include_proc_macro::macros;

macros!(
    // A function-like macro. Declares `mod parsing` and forwards to it.
    function(sql) -> parsing::parse_sql,
    // The name can be left out, and is then taken from the last path segment,
    // so this one is called `tokenize`.
    function -> parsing::tokenize,
    // `parsing` is already declared by the first entry, so say `use` rather
    // than declaring it a second time.
    function(lex) -> use parsing::lex,

    // An attribute macro. Its implementation takes two token streams.
    attribute(instrument) -> tracing_impl::instrument,
    // Modules nest to any depth.
    attribute(cached) -> caching::memo::store::apply,

    // A derive. The name in parentheses is what deriving types write.
    derive(Builder) -> derives::builder,
    // Helper attributes go beside it, and there can be any number.
    derive(Validate, attributes(required, length, range)) -> derives::validate,

    // The implementation can also be a file that is not part of the module tree,
    // named relative to this one.
    function(greet) -> "impls/hello.rs"::greet,
    // Or relative to the crate root, with a leading `@`.
    derive(Display) -> @"tests/fixtures/display.rs"::display,
);
```

The implementations are ordinary functions. They carry no `#[proc_macro]` attribute,
because they cannot: that attribute is only legal at the crate root, which is the
restriction being worked around.

```rust,ignore
// parsing.rs
use proc_macro::TokenStream;

pub fn parse_sql(input: TokenStream) -> TokenStream { /* ... */ }
pub fn tokenize(input: TokenStream) -> TokenStream { /* ... */ }
```

### Where an implementation can be

The same forms work for all three kinds of macro and in every position, because they are
read by one shared piece of the crate rather than reimplemented per kind.

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

Leaving the macro name out is available for `function` and `attribute`, and takes the
name from the path's last segment. It is refused for a path that is a bare name, because
the generated item would then take that same name in that same scope and shadow the
function it means to call. A derive always names itself, since the name is what the
deriving type writes.

### The single forms

`macros!` is a convenience over three macros that do one declaration each, and they are
available directly when that reads better:

```rust,ignore
include_proc_macro::proc_macro!(sql -> parsing::parse_sql);
include_proc_macro::attr_macro!(instrument -> tracing_impl::instrument);
include_proc_macro::derive_macro!((Validate, attributes(required)) -> derives::validate);
```

## Why the examples say `ignore`

Every code block here is marked `rust,ignore`, and that is structural rather than
neglect. These macros expand to items carrying `#[proc_macro]`, which rustc accepts only
in a crate whose manifest sets `proc-macro = true`. A doctest is compiled as an ordinary
crate, so a doctest of this crate cannot be made to pass no matter what it contains.

What checks them instead is `arm_matrix/` and `arm_matrix_test/` in the repository: every
form in the table above, against every kind of macro and every way of declaring one, each
asserted to produce its own distinct output. The refusals have their own compile-fail
suite. A form that appears in this README and does not work is a failing build there.

## Runnable examples

Three of them, under `integration_test/examples/`, each runnable on its own:

```text
cargo run -p integration_test --example one_function_macro
cargo run -p integration_test --example all_three_kinds
cargo run -p integration_test --example every_path_form
```

The first is the smallest thing that works: one function-like macro, declared and called.
The second reaches all three kinds of macro from one declaration block. The third walks
every path form in the table above, one line of output per form.

They are run by `cargo test`, in `integration_test/tests/examples_run.rs`, which checks
what each one prints rather than only that it built.

## Features

Neither changes what the crate does, and both exist so a consumer can name them.

| Feature | Effect |
|---|---|
| `no_std` | Adds `#![no_std]`. The crate is `macro_rules!` only, so this is the attribute and nothing more. |
| `no_alloc` | Implies `no_std`. States what is already true: nothing here allocates. |

What a macro from here expands into is a `#[proc_macro]` entry point, and a proc-macro
crate cannot be `no_std` whatever this crate does, because it runs on the host inside the
compiler. `tests/feature_matrix.rs` builds under each selection and asserts the macros
still expand.

## What you would write otherwise

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

## Cost

All of it happens during expansion, so nothing reaches the compiled artifact that you
would not have written yourself, and crates that *use* your macros are unaffected: they
depend on your proc-macro crate, not on this one. Your own crate takes one more
dependency to compile, which has no dependencies of its own.

`macros!` recurses once per declaration rather than once per token, which keeps a long
list comfortably inside the default recursion limit of 128. A hundred declarations in a
single invocation is asserted in the test suite. Well past that, either split the list or
raise `#![recursion_limit]`.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> You can check out the full license [here](https://github.com/orgrinrt/include_proc_macro/blob/main/LICENSE)

This project is licensed under the terms of the **MPL-2.0** license.
