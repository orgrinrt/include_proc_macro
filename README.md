include_proc_macro
============
[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/stargazers) 
![Crates.io Total Downloads](https://img.shields.io/crates/d/include_proc_macro)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/include_proc_macro.svg)](https://github.com/orgrinrt/include_proc_macro/issues) 
[![Current Version](https://img.shields.io/badge/version-0.1.0-orange.svg)](https://github.com/orgrinrt/include_proc_macro) 

>A simple shorthand for including proc-macro source files in the module tree for IDEs or other similar purposes.

---

## Usage

The `include_proc_macro` crate provides a macro designed for easy integration with external tooling, particularly when working with procedural macros.

The macro checks if debug assertions are enabled (`#[cfg(debug_assertions)]`). If debug assertions are enabled, it includes a targeted .rs file from the Cargo project's root directory (obtained through the `CARGO_MANIFEST_DIR` environment variable) in the module tree.

## Example

In Rust:
```rust
include_proc_macro::include_proc_macro!("sample");
```

The above command includes `sample.rs` from the root of the Cargo project during a debug assertion (development time).

The main parameter of the macro is:

- `$file_name`: a string literal representing the name of the procedural macro source file (.rs) to be included in the source tree during development time (this helps to enable certain advanced IDE features).

Please note that this crate only contains the `include_proc_macro` macro. For more details, please refer to the macro definition inside the crate.

---

Additionally, include_proc_macro provides two convenient shorthand aliases, `here!` and `named!`:

```rust 
include_proc_macro::here!("sample"); 
```

Please note that using these aliases will yield the same result as directly using include_proc_macro. They are 
included for convenience and for prettier code (i.e for when you want to / have to use fully qualified paths).

>Note: 
> Seeing as (unused) procmacros do not introduce a compile-time or runtime overhead, and the namespace pollution 
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
