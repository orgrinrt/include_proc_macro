include_proc_macro
============
[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/stargazers) 
![Crates.io Total Downloads](https://img.shields.io/crates/d/include_proc_macro)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/issues) 
[![Current Version](https://img.shields.io/badge/version-1.0.0-orange.svg)](https://github.com/orgrinrt/include_proc_macro) 

>A simple shorthand for including proc-macro source files in the module tree for IDEs or other similar purposes.

---

## Usage

The `include_proc_macro` crate provides a macro designed for easy integration with external tooling, particularly 
when working with procedural macros. It's extremely simple, just wraps an `include!` macro call with some sugar, and is 
primarily useful to reduce 
boilerplate 
and prettify 
procmacro code.

The macro checks if debug assertions are enabled (`#[cfg(debug_assertions)]`). If debug assertions are enabled, it 
includes a targeted .rs file from the Cargo project's root directory (obtained through the `CARGO_MANIFEST_DIR` 
environment variable) in the module tree. Simple as that.

## Example

In Rust:
```rust
// lib.rs
include_proc_macro::include_proc_macro!("sample");
```

The above command includes `sample.rs` from the root of the Cargo project during a debug assertion (development time)
. It simply expands to:

```rust
#[cfg(debug_assertions)]
include!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "sample", // <-- arg
        ".rs"
    )
);
```

The main parameter of the macro is:

- `$file_name`: a string literal representing the name of the procedural macro source file (.rs) to be included in the source tree during development time (this helps to enable certain advanced IDE features).

### In practice

You'll want to call the macro wherever your environment finds it for the module tree.

Normally this could be, for example, the lib.rs file of the proc-macro crate.

---

## The problem

For some IDEs or other kinds of programming environments, the module tree is what brings auto-completion
and other kinds of helpful bits of sugar to the developer and keeps it in sync.

Proc-macro crates, however, are not included in the module tree ordinarily, and have their source
files in the root of the crate as opposed to lib.rs, so not every environment or application can
detect them or make sense of them.

This is entirely dependent on situation, but one way of achieving this is to include the proc-macro
code in a lib.rs file for the proc-macro crate, which makes certain IDEs or other applications "find" it
and give auto-completion suggestions and documentation etc. for when one works with the proc-macro.

This might not be what you want, and maybe some other way is more suitable for your situation. However,
this crate exists as a simple way to bridge the gap and allow external tooling to find the connection
between the proc-macro crate and rest of the code.



---
## Extras
Additionally, include_proc_macro provides two convenient shorthand aliases, `here!` and `named!`:

```rust 
include_proc_macro::here!("sample"); 
```

Please note that using these aliases will yield the same result as directly using include_proc_macro. They are 
included for convenience and for prettier code (i.e for when you want to / have to use fully qualified paths).

>Note: 
> Seeing as (unused) macros do not introduce a compile-time or runtime overhead, and the namespace pollution 
> is both minimal and unlikely to clash or otherwise cause problems, having these 
> aliases seems okay to me.
However, if it turns out to be undesirable, we'll hide these behind a feature flag.

---

## Buy me a coffee

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

---

## License
>You can check out the full license [here](https://github.com/orgrinrt/include_proc_macro/blob/master/LICENSE)

This project is licensed under the terms of the **MIT** license.
