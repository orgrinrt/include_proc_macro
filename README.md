include_proc_macro
============

<div style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/stargazers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/include_proc_macro)](https://crates.io/crates/include_proc_macro)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/issues)
[![Current Version](https://img.shields.io/badge/version-2.0.1-blue.svg)](https://github.com/orgrinrt/include_proc_macro)

> A convenient macro for working with multiple procedural macros in one crate, and to import them from any arbitrary paths. Reduces boilerplate and repetition, and improves readability.

</div>

## Usage

The
`include_proc_macro` crate provides utilities that make working with procedural macros simpler and more convenient. It offers a simple, comparatively pretty syntax for defining multiples of function-like macros, attribute macros, and derive macros, in a single crate, along with flexible options for importing their implementations.

### Breaking changes in 2.0.0

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
// which still resulted in you having to be verbose or otherwise tricky with other macros
// in the crate's .module tree
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
    // paths at custom source dir within the crate, such as tests
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
    function -> "path/to/file"::function_name,
    // with `@` prefix for paths relative to crate root
    attribute -> @"custom/src_dir"::attr_function,
    // the path can be absolute too, but there are considerations outside of this 
    // crate's scope to think over. go wild I suppose
    derive(DefaultImpl) -> "/users/user/dev/macros"::default_impl
);
```

Though it doesn't look like much, this would save you *a
lot* of boilerplate, though the average case would likely not have so many macros defined in a single crate. But hey, you can do it if you want to, and now it won't look like a mess.

### In practice

This crate reduces the boilerplate needed when working with procedural macros, especially if there are many of them in a large codebase.

Instead of writing out each proc macro definition by explicitly delegating to its implementation, in the crate root, repetitively, with all the proper attributes and function signatures, you can instead just use the
`macros!` syntax to define them all without thinking about the boilerplate.

Also, the ability to fairly cleanly import implementations from external files can be useful for some use cases, such as when you want to keep your macro implementations separate from the main codebase, for whatever reason, have procedural macro tests and want to organize them better, or something wild like allow for external proc macro injection.

## The problem

Rust's procedural macro system requires all procedural macros to be defined at the crate root. This can lead to a gigantic, hard-to-navigate root module, and even if avoiding that some way, e.g by separating impls from the macro declarations like this crate does under-the-hood, it's all still very verbose and repetitive to write. It's a little bit tedious to organize larger proc macro codebases with multiple kinds of macro implementations, if you want to keep them in the same crate.

This crate solves these problems by:

1. Providing a concise, declarative syntax for defining all types of procedural macros
2. Supporting imports from various module paths and even external files
3. Allowing for custom naming of macros separate from their implementation
4. Enabling batch definitions for much prettier and more readable root module

This is all done via the macro, at compile time, so there are no runtime overhead or other similar implications to consider. The compilation time is slightly increased (due to this dependency), but this is of course only for your proc macro crate, and not for the actual code that uses the macros. For most use cases, you won't notice any side effects.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> You can check out the full license [here](https://github.com/orgrinrt/include_proc_macro/blob/master/LICENSE)

This project is licensed under the terms of the **MIT** license.
