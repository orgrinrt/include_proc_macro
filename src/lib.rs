//! This crate provides the `include_proc_macro` macro.
//!
//! The `include_proc_macro` macro is designed with a single, focused purpose: to aid in integrating
//! with external tooling, particularly when working with procedural macros.
//!
//! This macro checks if debug assertions are enabled (`#[cfg(debug_assertions)]`).
//! If so, it includes a targeted .rs file from the Cargo project's root directory
//! (obtained through the `CARGO_MANIFEST_DIR` environment variable) in the module tree.
//!
//! ## Example
//! ```rust
//! include_proc_macro::include_proc_macro!("sample");
//! ```
//! The command above includes `sample.rs` from the root of the Cargo project during a debug
//! assertion (development time).
//!
//! The macro's key parameter:
//! - `$file_name`: a string literal representing the name of the proc macro source file (.rs) to be included in the source tree during development time (this helps to enable certain advanced IDE features).
//!
//! Please note that this crate only contains the `include_proc_macro` macro.
//!
//! For more details, please see the macro definition inside the crate.

#[macro_export]
/// `include_proc_macro` is a macro with single, focused purpose: help integrating with external
/// tooling in certain situations, when working with procedural macros specifically. It takes a
/// single parameter:
/// - `$file_name`: a string literal that represents the name of the proc macro source file (.rs)
/// to be
/// included in the source tree during dev time (to enable certain more advanced IDE features).
///
/// This macro checks if debug assertions are enabled (`#[cfg(debug_assertions)]`),
/// if so, it includes the targeted .rs file from the Cargo project's root directory
/// (obtained from `CARGO_MANIFEST_DIR` environment variable) in the module tree.
///
/// # Examples
/// ```
/// include_proc_macro::include_proc_macro!("sample");
/// ```
/// The above line includes `sample.rs` from the root of the Cargo project during a debug
/// assertion (i.e dev time).

macro_rules! include_proc_macro {
    ($file_name:expr) => {
        #[cfg(debug_assertions)]
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/",
            $file_name,
            if $file_name.ends_with(".rs") { "" } else { ".rs" },
        ));
    };
}

#[macro_export]
/// `here` is an alias of [`include_proc_macro`](crate::include_proc_macro) and serves as shorthand
/// when working with procedural macros specifically at a local scope. It has the same parameter as
/// its original:
/// - `$file_name`: a string literal that represents the name of the proc macro source file (.rs)
/// to be included in the source tree during dev time.
///
/// This alias only makes sense when used inline, such as `include_proc_macro::here!("...")`.
/// If you intend to use this macro, consider using it fully qualified to avoid unexpected
/// name clashes if brought into global namespace by `use include_proc_macro::*`.
///
/// # Examples
/// ```
/// include_proc_macro::here!("sample");
/// ```
/// The above line includes `sample.rs` from the root of the Cargo project during a debug
/// assertion (i.e dev time).
///
/// See: [`include_proc_macro`](crate::include_proc_macro)
macro_rules! here {
    ($file_name:expr) => {
        include_proc_macro!($file_name);
    };
}

#[macro_export]
/// `named` is an alias of [`include_proc_macro`](crate::include_proc_macro) and serves as another
/// shorthand when working with procedural macros specifically to provide more descriptive usage.
/// It takes the same parameter as its original:
/// - `$file_name`: a string literal that represents the name of the proc macro source file (.rs)
/// to be included in the source tree during dev time.
///
/// Similar to `here`, this alias is recommended to be used fully qualified such as
/// `include_proc_macro::named!("...")`, to prevent naming conflicts with `use include_proc_macro::*`.
///
/// # Examples
/// ```
/// include_proc_macro::named!("sample");
/// ```
/// The above line includes `sample.rs` from the root of the Cargo project during a debug
/// assertion (i.e dev time).
///
/// See: [`include_proc_macro`](crate::include_proc_macro)
macro_rules! named {
    ($file_name:expr) => {
        include_proc_macro!($file_name);
    };
}
