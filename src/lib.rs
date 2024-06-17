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
        include!(concat!(env!(
            "CARGO_MANIFEST_DIR"), "/", 
        $file_name, if $file_name.ends_with(".rs") { "" } else { ".rs" }
        ,));
    };
}
