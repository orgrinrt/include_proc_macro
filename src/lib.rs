#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "README.md"))]

#[macro_export]
/// Includes a source file from crate root in the module tree.
/// It takes a single parameter:
/// - `$file_name`: a string literal that represents the name of the source file (.rs)
/// to be included in the module tree during dev time (to enable certain more advanced IDE
/// features).
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
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $file_name, ".rs"));
    };
}

#[macro_export]
/// `here!` is an alias of [`include_proc_macro`](crate::include_proc_macro) and serves as
/// a shorthand for when you want to avoid polluting the global namespace with ::* or don't
/// want to or need to write explicit use statements for a single use of this macro.
///
/// > This alias only makes sense when used inline, such as `include_proc_macro::here!("...")`.
/// > If you intend to use this macro, consider using it fully qualified as intended.
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
        $crate::include_proc_macro!($file_name);
    };
}

#[macro_export]
/// `named!` is an alias of [`include_proc_macro`](crate::include_proc_macro) and serves as
/// a shorthand for when you want to avoid polluting the global namespace with ::* or don't
/// want to or need to write explicit use statements for a single use of this macro.
///
/// > This alias only makes sense when used inline, such as `include_proc_macro::named!("...")`.
/// > If you intend to use this macro, consider using it fully qualified as intended.
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
        $crate::include_proc_macro!($file_name);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    include_proc_macro!("tests/hello");

    #[test]
    fn test_hello_macro() {
        let h = hello();

        assert_eq!("Hello, world!", h);
    }
}
