//! Why every example in the readme says `ignore`.
//!
//! These macros expand to items carrying `#[proc_macro]`, which rustc accepts only in a
//! crate whose manifest sets `proc-macro = true`. A doctest is compiled as an ordinary
//! crate, and so is this file, so both refuse for the same reason.
//!
//! The readme states that. This is what makes it a fact rather than an assertion.

pub fn implementation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

include_proc_macro::proc_macro!(demo -> use implementation);

fn main() {}
