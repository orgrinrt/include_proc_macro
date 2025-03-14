#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "README.md"))]

/// Defines and delegates a function-like procedural macro from another module.
///
/// ## Usage patterns:
/// - Already in scope: `proc_macro!(name -> function)`
/// - Module reference: `proc_macro!(name -> module::function)`
/// - Nested modules: `proc_macro!(name -> a::b::c::function)`
/// - Literal path: `proc_macro!(name -> "path/to/file.rs"::function)`
/// - Crate-relative path: `proc_macro!(name -> @"path/from/crate/root.rs"::function)`
#[macro_export]
macro_rules! proc_macro {
    // direct function reference
    ($name:ident -> $func:ident) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $func(input)
        }
    };

    // module reference
    ($name:ident -> $module:ident :: $func:ident) => {
        mod $module;

        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $module::$func(input)
        }
    };

    // nested
    ($name:ident -> $($module:ident)::+ :: $func:ident) => {
        $(mod $module;)*

        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($module::)+$func(input)
        }
    };

    // regular path (absolute or relative to current file)
    ($name:ident -> $path:literal :: $func:ident) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            #[path = $path]
            mod __inner;
            __inner::$func(input)
        }
    };

    // crate-relative path (literal path prepended by `@`)
    ($name:ident -> @$path:literal :: $func:ident) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            mod __inner {
                include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
            }
            __inner::$func(input)
        }
    };
}

/// Defines and delegates a attribute macro from another module.
///
/// ## Usage patterns:
/// - Already in scope: `attr_macro!(name -> function)`
/// - Module reference: `attr_macro!(name -> module::function)`
/// - Nested modules: `attr_macro!(name -> a::b::c::function)`
/// - Literal path: `attr_macro!(name -> "path/to/file.rs"::function)`
/// - Crate-relative path: `attr_macro!(name -> @"path/from/crate/root.rs"::function)`
#[macro_export]
macro_rules! attr_macro {
    // direct function reference
    ($name:ident -> $func:ident) => {
        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $func(attr, item)
        }
    };

    // module reference
    ($name:ident -> $module:ident :: $func:ident) => {
        mod $module;

        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $module::$func(attr, item)
        }
    };

    // nested
    ($name:ident -> $($module:ident)::+ :: $func:ident) => {
        $(mod $module;)*

        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($module::)+$func(attr, item)
        }
    };

    // regular path
    ($name:ident -> $path:literal :: $func:ident) => {
        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            #[path = $path]
            mod __inner;
            __inner::$func(attr, item)
        }
    };

    // crate-relative path (prefixed with @)
    ($name:ident -> @$path:literal :: $func:ident) => {
        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            mod __inner {
                include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
            }
            __inner::$func(attr, item)
        }
    };
}

/// Defines and delegates a derive macro from another module.
///
/// ## Usage patterns:
/// - Already in scope: `derive_macro!(Name -> function)`
/// - Module reference: `derive_macro!(Name -> module::function)`
/// - Nested modules: `derive_macro!(Name -> a::b::c::function)`
/// - Literal path: `derive_macro!(Name -> "path/to/file.rs"::function)`
/// - Crate-relative path: `derive_macro!(Name -> @"path/from/crate/root.rs"::function)`
#[macro_export]
macro_rules! derive_macro {
    // direct function reference
    ($name:ident -> $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $func(input)
        }
    };

    // module reference
    ($name:ident -> $module:ident :: $func:ident) => {
        mod $module;

        #[allow(non_snake_case)]
        #[proc_macro_derive($name)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $module::$func(input)
        }
    };

    // nested
    ($name:ident -> $($module:ident)::+ :: $func:ident) => {
        $(mod $module;)*

        #[allow(non_snake_case)]
        #[proc_macro_derive($name)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($module::)+$func(input)
        }
    };

    // regular path
    ($name:ident -> $path:literal :: $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            #[path = $path]
            mod __inner;
            __inner::$func(input)
        }
    };

    // crate-relative path (prefixed with @)
    ($name:ident -> @$path:literal :: $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            mod __inner {
                include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
            }
            __inner::$func(input)
        }
    };
}

/// Delegates procedural macro declarations to implementation modules.
///
/// Note that the implementation
/// modules should not (and can not, which is why this crate exists) include the
/// proc macro attributes (`#[proc_macro]`, `#[proc_macro_attribute]`, or `#[proc_macro_derive]`).
///
/// Uses the specialized macros internally: `proc_macro!`, `attr_macro!`, and `derive_macro!`.
///
/// ## Supported syntax:
/// - `function(macro_name) -> impl`: Function-like proc macros with custom name
/// - `function -> impl`: Function-like proc macros using function name as macro name
/// - `attribute(attr_name) -> impl`: Attribute proc macros with custom name
/// - `attribute -> impl`: Attribute proc macros using function name as macro name
/// - `derive(DeriveName) -> impl`: Derive proc macros
///
/// Where `impl` can be:
/// - Direct function: `function`
/// - Module reference: `module::function`
/// - Nested modules: `a::b::c::function`
/// - Literal path: `"path/to/file.rs"::function`
/// - Crate-relative path: `@"path/from/crate/root.rs"::function`
///
/// ## Examples
/// ```rust
/// include_proc_macro::macros!(
///     function(foo) -> foo_mod::implement,
///     function -> bar_mod::bar, // uses `bar` as the macro name
///     function(baz) -> existing_function,  // `existing_function` is already in scope, just use directly
///     function(buzz) -> "src/impls/hello.rs"::hello,
///     function -> @"custom_src/impls/world.rs"::world,  // impl in custom crate path, uses 'world' as the macro name
///     attribute(my_attr) -> attrs::process,
///     attribute -> attrs::custom,  // uses 'custom' as the macro name
///     derive(MyDerive) -> derives::generate // at least for now, derive macros require the derive name
/// );
/// ```
#[macro_export]
macro_rules! macros {
    () => {};

    // with trailing comma
    (function -> $module:ident :: $func:ident, $($tail:tt)*) => {
        $crate::proc_macro!($func -> $module::$func);
        $crate::macros!($($tail)*);
    };

    (attribute -> $module:ident :: $func:ident, $($tail:tt)*) => {
        $crate::attr_macro!($func -> $module::$func);
        $crate::macros!($($tail)*);
    };

    (function($name:ident) -> $func:ident, $($tail:tt)*) => {
        $crate::proc_macro!($name -> $func);
        $crate::macros!($($tail)*);
    };

    (function($name:ident) -> $module:ident :: $func:ident, $($tail:tt)*) => {
        $crate::proc_macro!($name -> $module::$func);
        $crate::macros!($($tail)*);
    };

    (function($name:ident) -> $path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::proc_macro!($name -> $path::$func);
        $crate::macros!($($tail)*);
    };

    (attribute($name:ident) -> $func:ident, $($tail:tt)*) => {
        $crate::attr_macro!($name -> $func);
        $crate::macros!($($tail)*);
    };

    (attribute($name:ident) -> $module:ident :: $func:ident, $($tail:tt)*) => {
        $crate::attr_macro!($name -> $module::$func);
        $crate::macros!($($tail)*);
    };

    (attribute($name:ident) -> $path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::attr_macro!($name -> $path::$func);
        $crate::macros!($($tail)*);
    };

    (derive($name:ident) -> $func:ident, $($tail:tt)*) => {
        $crate::derive_macro!($name -> $func);
        $crate::macros!($($tail)*);
    };

    (derive($name:ident) -> $module:ident :: $func:ident, $($tail:tt)*) => {
        $crate::derive_macro!($name -> $module::$func);
        $crate::macros!($($tail)*);
    };

    (function -> $path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::proc_macro!($func -> $path::$func);
        $crate::macros!($($tail)*);
    };

    (attribute -> $path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::attr_macro!($func -> $path::$func);
        $crate::macros!($($tail)*);
    };

    (derive($name:ident) -> $path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::derive_macro!($name -> $path::$func);
        $crate::macros!($($tail)*);
    };

    (function -> @$path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::proc_macro!($func -> @$path::$func);
        $crate::macros!($($tail)*);
    };

    (attribute -> @$path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::attr_macro!($func -> @$path::$func);
        $crate::macros!($($tail)*);
    };

    (function($name:ident) -> @$path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::proc_macro!($name -> @$path :: $func);
        $crate::macros!($($tail)*);
    };

    (attribute($name:ident) -> @$path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::attr_macro!($name -> @$path :: $func);
        $crate::macros!($($tail)*);
    };

    (derive($name:ident) -> @$path:literal :: $func:ident, $($tail:tt)*) => {
        $crate::derive_macro!($name -> @$path :: $func);
        $crate::macros!($($tail)*);
    };

    // without trailing comma
    (function -> $module:ident :: $func:ident) => {
        $crate::proc_macro!($func -> $module::$func);
    };

    (attribute -> $module:ident :: $func:ident) => {
        $crate::attr_macro!($func -> $module::$func);
    };

    (function($name:ident) -> $func:ident) => {
        $crate::proc_macro!($name -> $func);
    };

    (function($name:ident) -> $module:ident :: $func:ident) => {
        $crate::proc_macro!($name -> $module::$func);
    };

    (function($name:ident) -> $path:literal :: $func:ident) => {
        $crate::proc_macro!($name -> $path::$func);
    };

    (attribute($name:ident) -> $func:ident) => {
        $crate::attr_macro!($name -> $func);
    };

    (attribute($name:ident) -> $module:ident :: $func:ident) => {
        $crate::attr_macro!($name -> $module::$func);
    };

    (attribute($name:ident) -> $path:literal :: $func:ident) => {
        $crate::attr_macro!($name -> $path::$func);
    };

    (derive($name:ident) -> $func:ident) => {
        $crate::derive_macro!($name -> $func);
    };

    (derive($name:ident) -> $module:ident :: $func:ident) => {
        $crate::derive_macro!($name -> $module::$func);
    };

    (function -> $path:literal :: $func:ident) => {
        $crate::proc_macro!($func -> $path::$func);
    };

    (attribute -> $path:literal :: $func:ident) => {
        $crate::attr_macro!($func -> $path::$func);
    };

    (derive($name:ident) -> $path:literal :: $func:ident) => {
        $crate::derive_macro!($name -> $path::$func);
    };

    (function -> @$path:literal :: $func:ident) => {
        $crate::proc_macro!($func -> @$path::$func);
    };

    (attribute -> @$path:literal :: $func:ident) => {
        $crate::attr_macro!($func -> @$path::$func);
    };

    (function($name:ident) -> @$path:literal :: $func:ident) => {
        $crate::proc_macro!($name -> @$path :: $func);
    };

    (attribute($name:ident) -> @$path:literal :: $func:ident) => {
        $crate::attr_macro!($name -> @$path :: $func);
    };

    (derive($name:ident) -> @$path:literal :: $func:ident) => {
        $crate::derive_macro!($name -> @$path :: $func);
    };
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_integration_crate() {
        let status = Command::new("cargo")
            .args(&["test", "-p", "integration_test"])
            .status()
            .expect("Failed to run integration tests");

        assert!(status.success(), "Integration tests failed");
    }
}
