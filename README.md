include_proc_macro
============

<div style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/stargazers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/include_proc_macro)](https://crates.io/crates/include_proc_macro)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/issues)
[![Current Version](https://img.shields.io/badge/version-1.1.1-blue.svg)](https://github.com/orgrinrt/include_proc_macro)

> A convenient shorthand for working with multiple procedural macros in one crate, also to import them from any arbitrary paths. Reduces boilerplate and repetition, and improves readability.

</div>

## Usage

The
`include_proc_macro` crate provides utilities that make working with procedural macros simpler and more convenient. It offers a simple, comparatively pretty syntax for defining function-like macros, attribute macros, and derive macros, along with flexible options for importing their implementations.

### Breaking Changes in 2.0.0

Version 2.0.0 completely overhauls the api and the way the macros are used:

```rust
// old:
include_proc_macro!::include_proc_macro!(
    "some/path/to/file",
    alternatively / using / idents
);
// this would often cause name clashes, competing implementations
// and other issues, and was fairly unusable/unneeded outside of very niche applications.
// it was also only for including external macros from arbitrary paths,
// which still resulted in you having to be verbose with those in the macro crate's 
// module tree
```

For better readability, increased control, and making use of different types of proc macros in a single crate easier, the syntax evolved thus:

```rust
// new:
include_proc_macro!::macros!(
    // literal paths are still supported (relative and absolute)
    function -> "old/style/literal/path/inclusion"::macro_impl,
    // "just works" with normal module paths
    attribute -> any::amount::of:nested::mods::attr_impl,
    // with a `@` prefix we can more conveniently include macro implementations from 
    // paths outside the source directory, such as tests
    derive(MacroName) -> @"this/path/is/relative/to/crate/root"::derive_impl 
);
// now we hide implementation details within modules, and delegate to them
// within the crate root. should be fairly clash-free, and handles
// pretty much all the macros you'd want in a single proc-macro crate
```

## Example

```rust
use include_proc_macro::macros;

// we can define multiple macros in a single go, separated by commas
macros!(
    // for normal function-like proc macros we use `function`
    function -> implement::generate_function,
    // can define explicit custom macro names. here the macro would be `my_macro_name`
    // (otherwise we just inherit the name of the function)
    function(my_macro_name) -> implement::another_function, 

    // with the `attribute` keyword, we can define attribute macros
    attribute -> attr_impl::generate_attr,
    attribute(custom_attr) -> attr_impl::custom_implementation,

    // `derive` is for derive macros, and the name in parentheses is the actual derive name
    // (the function name will be inherited from source module, but is seldom needed)
    derive(DebugImpl) -> derive_impl::implement_debug,
    derive(DisplayImpl) -> derive_impl::implement_display,

    // include external files like so: 
    function -> "path/to/file" :: function_name,
    // with `@` prefix for paths relative to crate root
    attribute -> @"another/path" :: attr_function,
    // the path can be absolute too, but there are considerations outside of this 
    // crate's scope to think over. go wild I suppose
    derive(DefaultImpl) -> "/users/user/dev/macros" :: default_impl
);
```

Though it doesn't look like much, this would save you *a
lot* of boilerplate, though the average case would likely not have so many macros defined in a single crate. But hey, you can do it if you want to, and now it won't look like a mess.

### In practice

This crate significantly reduces the boilerplate needed when creating procedural macros. Instead of writing out each proc macro definition, repetitively, with their proper attributes and function signatures, you can use the concise
`macros!` syntax to define them all without thinking about the boilerplate.

The ability to import implementations from external files can be useful for some use cases, such as when you want to keep your macro implementations separate from the main codebase, for whatever reason, or allow for external codegen injection, which you probably shouldn't do, but you can, and now it's prettier too.

## The problem

Rust's procedural macro system requires all procedural macros to be defined at the crate root with specific attributes (`#[proc_macro]`,
`#[proc_macro_attribute]`, or
`#[proc_macro_derive]`). This can lead to gigantic, hard-to-navigate root module, and even if avoiding that by doing it like this crate does under-the-hood, still very verbose and repetitive code. Rhe restriction that proc macro attributes can only be applied at the crate root makes it challenging to organize larger codebases with multiple macro implementations.

This crate solves these problems by:

1. Providing a concise, declarative syntax for defining all types of procedural macros
2. Supporting imports from various module paths and even external files
3. Allowing for custom naming of macros separate from their implementation functions
4. Enabling batch definitions to reduce repetition

This is all done via the macro, at compile time, so there are no runtime overhead or other similar implications to consider. The compilation time is slightly increased (due to this dependency), but this is of course only for your proc macro crate, and not for the actual code that uses the macros. For most use cases, you won't notice any side effects.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> You can check out the full license [here](https://github.com/orgrinrt/include_proc_macro/blob/master/LICENSE)

This project is licensed under the terms of the **MIT** license.
