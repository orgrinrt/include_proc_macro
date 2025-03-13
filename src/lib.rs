#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "README.md"))]

#[macro_export]
/// Includes a source file from crate root in the module tree.
/// It supports several formats:
///
/// - `$module:ident`: A single identifier representing the module name, which is also used as the file path
/// - `$dir:ident / $module:ident`: Directory and module name as identifiers
/// - `$first:ident / $second:ident / $($rest:ident)/+`: Nested path structure with multiple identifiers
/// - `$module_name:ident = $path:literal`: Explicit module name with a string literal path
/// - Multiple entries: Comma-separated list of any of the above formats
///
/// This macro checks if debug assertions are enabled (`#[cfg(debug_assertions)]`),
/// if so, it includes the targeted .rs file from the Cargo project's root directory
/// (obtained from `CARGO_MANIFEST_DIR` environment variable) in the module tree.
///
/// ##### Breaking Changes in 1.1.0
/// Direct string literal paths (e.g., `include_proc_macro!("sample")`) are no longer supported.
/// You must either use explicit module naming or path identifiers instead.
///
/// # Examples
/// ```
/// // Module name same as file name
/// include_proc_macro::include_proc_macro!(sample);
///
/// // Module in a subdirectory
/// include_proc_macro::include_proc_macro!(tests/hello);
///
/// // Custom module name with explicit path
/// include_proc_macro::include_proc_macro!(my_module = "tests/sample");
///
/// // Multiple modules at once
/// include_proc_macro::include_proc_macro!(sample, tests/hello, my_module = "tests/sample");
/// ```
macro_rules! include_proc_macro {
    ($module:ident) => {
        #[cfg(debug_assertions)]
        pub mod $module {
            include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", stringify!($module), ".rs"));
        }
    };

    ($dir:ident / $module:ident) => {
        #[cfg(debug_assertions)]
        pub mod $module {
            include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", stringify!($dir), "/", stringify!($module), ".rs"));
        }
    };

    ($first:ident / $second:ident / $($rest:ident)/+) => {
        #[cfg(debug_assertions)]
        pub mod $($rest)/+ {
            include!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/",
                stringify!($first),
                "/",
                stringify!($second),
                $(concat!("/", stringify!($rest)))+,
                ".rs"
            ));
        }
    };

    ($path:literal) => {
        compile_error!("Since 1.1.0, the path should either explicitly start with a module name  (module_name = \"dir/path_as_str_lit\") or be entered without the string delims \"\" of a string literal (dir/path_as_str_lit).");
    };

    ($module_name:ident = $path:literal) => {
        #[cfg(debug_assertions)]
        pub mod $module_name {
            include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path, ".rs"));
        }
    };

    // multis

    ($module:ident, $($tail:tt)*) => {
        $crate::include_proc_macro!($module);
        $crate::include_proc_macro!($($tail)*);
    };

    ($dir:ident / $module:ident, $($tail:tt)*) => {
        $crate::include_proc_macro!($dir / $module);
        $crate::include_proc_macro!($($tail)*);
    };

    ($first:ident / $second:ident / $($rest:ident)/+, $($tail:tt)*) => {
        $crate::include_proc_macro!($first / $second / $($rest)/+);
        $crate::include_proc_macro!($($tail)*);
    };

    ($module_name:ident = $path:literal, $($tail:tt)*) => {
        $crate::include_proc_macro!($module_name = $path);
        $crate::include_proc_macro!($($tail)*);
    };
}

#[macro_export]
/// `named!` is an alias of [`include_proc_macro`](crate::include_proc_macro) and serves as
/// a shorthand for when you want to avoid polluting the global namespace with ::* or don't
/// want to or need to write explicit use statements for a single use of this macro.
///
/// > This alias only makes sense when used inline, such as `include_proc_macro::named!(..)`.
/// > If you intend to use this macro, consider using it fully qualified as intended.
///
/// ##### Breaking Changes in 1.1.0
/// Direct string literal paths (e.g., `named!("sample")`) are no longer supported.
/// You must either use explicit module naming or path identifiers instead.
///
/// # Examples
/// ```
/// // Module name same as file name
/// include_proc_macro::named!(sample);
///
/// // Module in a subdirectory
/// include_proc_macro::named!(tests/hello);
///
/// // Custom module name with explicit path
/// include_proc_macro::named!(my_module = "tests/sample");
///
/// // Multiple modules at once
/// include_proc_macro::named!(sample, tests/hello, my_module = "tests/sample");
/// ```
///
/// See: [`include_proc_macro`](crate::include_proc_macro)
macro_rules! named {
    ($module:ident) => {
        $crate::include_proc_macro!($module);
    };

    ($dir:ident / $module:ident) => {
        $crate::include_proc_macro!($dir / $module);
    };

    ($first:ident / $second:ident / $($rest:ident)/+) => {
        $crate::include_proc_macro!($first / $second / $($rest)/+);
    };

    ($path:literal) => {
        $crate::include_proc_macro!($path);
    };

    ($module_name:ident = $path:literal) => {
        $crate::include_proc_macro!($module_name = $path);
    };

    ($($all:tt)+) => {
        $crate::include_proc_macro!($($all)+);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    include_proc_macro!(tests / hello);

    #[test]
    fn test_hello_macro() {
        use crate::tests::hello::hello;

        let h = hello();

        assert_eq!("Hello, world!", h);
    }

    include_proc_macro!(tests / subdir / foo);

    #[test]
    fn test_foo_macro() {
        use crate::tests::foo::bar;

        let foo = bar();

        assert_eq!("baz", foo);
    }

    include_proc_macro!(literal = "tests/literal");

    #[test]
    fn test_macro_lit_str() {
        use crate::tests::literal::literal_str;

        let baz = literal_str();

        assert_eq!("ofo", baz);
    }

    include_proc_macro!(tests / subdir / another_literal);

    #[test]
    fn test_another_lit_str() {
        use crate::tests::another_literal::another_literal_str;

        let ofo = another_literal_str();

        assert_eq!("bra", ofo);
    }
}

#[cfg(test)]
mod multi_tests {
    use super::*;

    include_proc_macro!(
        tests / hello,
        tests / subdir / foo,
        literal = "tests/literal",
        tests / subdir / another_literal
    );

    #[test]
    fn test_hello_macro() {
        use crate::multi_tests::hello::hello;

        let h = hello();

        assert_eq!("Hello, world!", h);
    }

    #[test]
    fn test_foo_macro() {
        use crate::multi_tests::foo::bar;

        let foo = bar();

        assert_eq!("baz", foo);
    }

    #[test]
    fn test_macro_lit_str() {
        use crate::multi_tests::literal::literal_str;

        let baz = literal_str();

        assert_eq!("ofo", baz);
    }

    #[test]
    fn test_another_lit_str() {
        use crate::multi_tests::another_literal::another_literal_str;

        let ofo = another_literal_str();

        assert_eq!("bra", ofo);
    }
}
