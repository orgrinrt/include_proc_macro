include_proc_macro
============

<div style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/stargazers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/include_proc_macro)](https://crates.io/crates/include_proc_macro)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/issues)
[![Current Version](https://img.shields.io/badge/version-1.2.1-blue.svg)](https://github.com/orgrinrt/include_proc_macro)

> A simple shorthand for including proc-macro source files in the module tree for external tooling like IDEs or other
> similar purposes.

</div>

## Usage

The
`include_proc_macro` crate provides a macro designed for easy integration with external tooling, particularly when working with procedural macros. It's extremely simple, just wraps an
`include!` macro call with some sugar, and is primarily useful to reduce boilerplate and prettify proc-macro code.

> Note: `include_proc_macro` is intended to be added as a `dependency`, though it is only a dev-time dependency. The
> macro itself returns nothing outside of debug assertions, and the compiler is intelligent enough to ignore it, so
> in practice it achieves the same, though it has to be available.

The macro checks if debug assertions are enabled (
`#[cfg(debug_assertions)]`). If debug assertions are enabled, it includes a targeted .rs file from the Cargo project's root directory (obtained through the
`CARGO_MANIFEST_DIR`
environment variable) in the module tree. Simple as that.

### Breaking Changes in 1.1.0

Version 1.1.0 introduces breaking changes in how paths are specified:

- Direct string literal paths (e.g., `include_proc_macro!("sample")`) are no longer supported
- You must either use path identifiers or explicit module naming with string literals

## Example

In Rust:

```rust
// Module name same as file name
include_proc_macro::include_proc_macro!(sample);

// Module in a subdirectory
include_proc_macro::include_proc_macro!(tests/hello);

// Custom module name with explicit path
include_proc_macro::include_proc_macro!(my_module = "tests/sample");

// From 1.1.1 onwards, you can do it all with a single macro call for even more pretty code
include_proc_macro::include_proc_macro!(
    sample,
    tests/hello,
    deep/nested/path,
    my_module = "tests/sample"
);
```

The above commands expand to something like:

```rust
#[cfg(debug_assertions)]
pub mod sample {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "sample", ".rs"));
}

#[cfg(debug_assertions)]
pub mod hello {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "tests", "/", "hello", ".rs"));
}

#[cfg(debug_assertions)]
pub mod my_module {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "tests/sample", ".rs"));
}

// For multiple modules, each is processed individually as above
```

The macro supports several formats:

- `$module:ident`: A single identifier representing the module name, which is also used as the file path
- `$dir:ident / $module:ident`: Directory and module name as identifiers
- `$first:ident / $second:ident / $($rest:ident)/+`: Nested path structure with multiple identifiers
- `$module_name:ident = $path:literal`: Explicit module name with a string literal path
- 1.1.1 Onwards also multiple entries separated by commas: Include multiple modules in a single macro call

### In practice

You'll want to use the macro in a location that your development tools recognize as part of the module tree. In many cases, this means using it in the
`lib.rs` file of the procedural macro crate.

> Note:
> Please remember that you should *not* use or depend on the procedural macro code being exposed beyond the confines
> of its crate. This configuration is designed to function in the reverse direction: it introduces features like
> auto-completion into the procedural macro crate during development. *That's the reason why we only include them during debug assertions.*

## The problem

For some IDEs or other kinds of programming environments, understanding the module tree is crucial as it improves the developer experience by providing features like auto-completion and stable syncing. This is situational, and the way these are handled vary, but sometimes proc-macros can be a problem.

Simply put, in Rust, the module tree is a hierarchical representation of your code's organization. Every Rust file can act as a module, with submodules nested within, creating a tree-like structure that evolves alongside your project.

Procedural macro crates do exist in this module tree. However, their source files often live in the root of the crate and not necessarily inside a
`lib.rs` file. Furthermore, they aren't necessarily explicitly declared as modules. These deviations from typical Rust code can cause difficulties for development tools that rely on a "conventional"
project structure.

This isn't a universal issue, as it relies heavily on how each tool attempts to interpret procedural macros.

One practical workaround is to include the procedural macro code inside a
`lib.rs` file within the procedural macro crate, particularly during dev time, so as to not cause problems on release (or other non-debug) builds. This decision might make the macros more accessible to certain development tools, potentially improving discovery, auto-completion, syntax highlighting, and documentation support.

**This is what this crate does.**

Alternatively, you could use the
`#[path]` attribute along a module definition to point to the source file(s) in the cargo root, which in practice achieves pretty much the same. Some IDEs and environments also just simply work well with proc-macros, so a workaround isn't necessary in the first place.

While the solution provided by this crate is simple and effective, it's not one-size-fits-all. The best method depends on multiple factors like your tooling setup, personal preference, and the specific needs of your project. Still, the existence of this crate provides a simple solution to the problem; to connect your procedural macro crate with the rest of your code, making it more discoverable by external tools.

## Extras

Additionally, include_proc_macro provides a convenient shorthand alias, `named!`:

```rust
// Module name same as file name
include_proc_macro::named!(sample);

// Module in a subdirectory
include_proc_macro::named!(tests/hello);

// Custom module name with explicit path
include_proc_macro::named!(my_module = "tests/sample");

// Multiple modules at once
include_proc_macro::named!(
    sample, 
    tests/hello, 
    my_module = "tests/sample"
);
```

Please note that using this alias will yield the same result as directly using include_proc_macro. It is included for convenience and for prettier code (i.e for when you want to / have to use fully qualified paths).

> Note:
> Seeing as (unused) macros do not introduce a compile-time or runtime overhead, and the namespace pollution
> is both minimal and unlikely to clash or otherwise cause problems, having these
> aliases seems okay to me.
> However, if it turns out to be undesirable, we'll hide these behind a feature flag.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> You can check out the full license [here](https://github.com/orgrinrt/include_proc_macro/blob/master/LICENSE)

This project is licensed under the terms of the **MIT** license.
