#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "README.md"))]

/// Defines and delegates a function-like procedural macro from another module.
///
/// ## Usage patterns:
/// - Already in scope: `proc_macro!(name -> function)`
/// - Module reference: `proc_macro!(name -> module::function)`
/// - Nested modules: `proc_macro!(name -> a::b::c::function)`
/// - Literal path: `proc_macro!(name -> "path/to/file.rs"::function)`
/// - Crate-relative path: `proc_macro!(name -> @"path/from/crate/root.rs"::function)`
///
/// See: [`macros!`](crate::macros)
#[macro_export]
macro_rules! proc_macro {
    // base implementation for direct function reference with explicit use keyword
    ($name:ident ->  use $func:ident) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $func(input)
        }
    };
    // shorthand that delegates to module pattern
    ($name:ident -> $func:ident) => {
        $crate::proc_macro!($name -> mod $func);
    };

    // base implementation for using existing modules
    ($name:ident -> use $module:ident :: $func:ident) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $module::$func(input)
        }
    };

    // declares module explicitly and delegates to use variant
    ($name:ident -> mod $module:ident :: $func:ident) => {
        mod $module;
        $crate::proc_macro!($name -> use $module :: $func);
    };

    // implicit module (defaults to explicit module declaration)
    ($name:ident -> $module:ident :: $func:ident) => {
        $crate::proc_macro!($name -> mod $module :: $func);
    };

    // base implementation for nested modules with existing imports
    ($name:ident -> use $($module:ident)::+ :: $func:ident) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($module::)+$func(input)
        }
    };

    // declares root module and delegates to use variant for nested modules
    ($name:ident -> mod $first:ident $(:: $rest:ident)+ :: $func:ident) => {
        mod $first;
        $crate::proc_macro!($name -> use $first$(::$rest)+ :: $func);
    };

    // implicit nested modules (defaults to explicit module declaration)
    ($name:ident -> $first:ident $(:: $rest:ident)+ :: $func:ident) => {
        $crate::proc_macro!($name -> mod $first $(:: $rest)+ :: $func);
    };

    // base implementation for literal file paths
    ($name:ident -> $path:literal :: $func:ident) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            #[path = $path]
            mod __inner;
            __inner::$func(input)
        }
    };

    // base implementation for crate-relative paths (prefixed with @)
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
///
/// See: [`macros!`](crate::macros)
#[macro_export]
macro_rules! attr_macro {
    // base implementation for direct function reference
    ($name:ident -> $func:ident) => {
        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $func(attr, item)
        }
    };

    // base implementation for using existing modules
    ($name:ident -> use $module:ident :: $func:ident) => {
        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $module::$func(attr, item)
        }
    };

    // declares module explicitly and delegates to use variant
    ($name:ident -> mod $module:ident :: $func:ident) => {
        mod $module;
        $crate::attr_macro!($name -> use $module :: $func);
    };

    // module reference (implicit mod - delegates to explicit mod)
    ($name:ident -> $module:ident :: $func:ident) => {
        $crate::attr_macro!($name -> mod $module :: $func);
    };

    // nested modules (use existing - base implementation)
    ($name:ident -> use $($module:ident)::+ :: $func:ident) => {
        #[proc_macro_attribute]
        pub fn $name(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($module::)+$func(attr, item)
        }
    };

    // nested modules (declare root module - delegates to use)
    ($name:ident -> mod $first:ident $(:: $rest:ident)+ :: $func:ident) => {
        mod $first;
        $crate::attr_macro!($name -> use $first$(::$rest)+ :: $func);
    };

    // nested modules (implicit mod - delegates to explicit mod)
    ($name:ident -> $first:ident $(:: $rest:ident)+ :: $func:ident) => {
        $crate::attr_macro!($name -> mod $first $(:: $rest)+ :: $func);
    };

    // path variants (base implementations since they create their own internal module anyway)
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
/// - With attributes: `derive_macro!((Name, attributes(attr1, attr2)) -> module::function)`
///
/// See: [`macros!`](crate::macros)
#[macro_export]
macro_rules! derive_macro {
    // -------------------------------------------------
    // -------------------------------------------------
    // base implementations for various pattern types
    // -------------------------------------------------

    // direct function reference (base implementation for general attributes)
    (($name:ident $(, $attr:tt)*) -> use $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name $(, $attr)*)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $func(input)
        }
    };

    // direct function reference (base implementation for helper attributes)
    (($name:ident, attributes($($attr:ident),*)) -> use $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name, attributes($($attr),*))]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $func(input)
        }
    };

    // module reference (use existing - base implementation for general attributes)
    (($name:ident $(, $attr:tt)*) -> use $module:ident :: $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name $(, $attr)*)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $module::$func(input)
        }
    };

    // module reference (use existing - base implementation for helper attributes)
    (($name:ident, attributes($($attr:ident),*)) -> use $module:ident :: $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name, attributes($($attr),*))]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $module::$func(input)
        }
    };

    // nested modules (use existing - base implementation for general attributes)
    (($name:ident $(, $attr:tt)*) -> use $($module:ident)::+ :: $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name $(, $attr)*)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($module::)+$func(input)
        }
    };

    // nested modules (use existing - base implementation for helper attributes)
    (($name:ident, attributes($($attr:ident),*)) -> use $($module:ident)::+ :: $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name, attributes($($attr),*))]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($module::)+$func(input)
        }
    };

    // -------------------------------------------------
    // delegating variants
    // -------------------------------------------------

    // direct function reference (delegates to use variant)
    (($name:ident $(, $attr:tt)*) -> $func:ident) => {
        $crate::derive_macro!(($name $(, $attr)*) -> use $func);
    };
    (($name:ident, attributes($($attr:tt),*)) -> $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),*)) -> use $func);
    };

    // module reference (declare module explicitly - delegates to use)
    (($name:ident $(, $attr:tt)*) -> mod $module:ident :: $func:ident) => {
        mod $module;
        $crate::derive_macro!(($name $(, $attr)*) -> use $module :: $func);
    };
    (($name:ident, attributes($($attr:tt),*)) -> mod $module:ident :: $func:ident) => {
        mod $module;
        $crate::derive_macro!(($name, attributes($($attr),*))-> use $module :: $func);
    };

    // module reference (implicit mod - delegates to explicit mod)
    (($name:ident $(, $attr:tt)*) -> $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name $(, $attr)*) -> mod $module :: $func);
    };
    (($name:ident, attributes($($attr:tt),*)) -> $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),*))-> mod $module :: $func);
    };

    // nested modules (declare root module - delegates to use)
    (($name:ident $(, $attr:tt)*) -> mod $first:ident $(:: $rest:ident)+ :: $func:ident) => {
        mod $first;
        $crate::derive_macro!(($name $(, $attr)*) -> use $first$(::$rest)+ :: $func);
    };
    (($name:ident, attributes($($attr:tt),*)) -> mod $first:ident $(:: $rest:ident)+ :: $func:ident) => {
        mod $first;
        $crate::derive_macro!(($name, attributes($($attr),*)) -> use $first$(::$rest)+ :: $func);
    };

    // nested modules (implicit mod - delegates to explicit mod)
    (($name:ident $(, $attr:tt)*) -> $first:ident $(:: $rest:ident)+ :: $func:ident) => {
        $crate::derive_macro!(($name $(, $attr)*) -> mod $first $(:: $rest)+ :: $func);
    };
    (($name:ident, attributes($($attr:tt),*)) -> $first:ident $(:: $rest:ident)+ :: $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),*)) -> mod $first $(:: $rest)+ :: $func);
    };

    // -------------------------------------------------
    // path variants (base implementations since they create their own module anyways)
    // -------------------------------------------------

    // path variants (base implementations since they create their own module anyway)
    (($name:ident $(, $attr:tt)*) -> $path:literal :: $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name $(, $attr)*)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            #[path = $path]
            mod __inner;
            __inner::$func(input)
        }
    };

    // crate-relative path (prefixed with @)
    (($name:ident $(, $attr:tt)*) -> @$path:literal :: $func:ident) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name $(, $attr)*)]
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
/// Uses the specialized macros internally: [`proc_macro!`](crate::proc_macro), [`attr_macro!`](crate::attr_macro), and [`derive_macro!`](crate::derive_macro).
///
/// ## Supported syntax:
/// - `function(macro_name) -> impl`: Function-like proc macros with custom name
/// - `function -> impl`: Function-like proc macros using function name as macro name
/// - `attribute(attr_name) -> impl`: Attribute proc macros with custom name
/// - `attribute -> impl`: Attribute proc macros using function name as macro name
/// - `derive(DeriveName) -> impl`: Derive proc macros
/// - `derive(DeriveName, attributes(attr1, attr2)) -> impl`: Derive macros with helper attributes
///
/// Where `impl` can be:
/// - Direct function: `function`
/// - Module reference with implicit module declaration (default): `module::function`
/// - Module reference with explicit declaration: `mod module::function`
/// - Module reference without declaration (already imported): `use module::function`
/// - Nested modules: `a::b::c::function`
/// - Literal path: `"path/to/file.rs"::function`
/// - Crate-relative path: `@"path/from/crate/root.rs"::function`
///
/// ## Examples
/// ```rust
/// include_proc_macro::macros!(
///     function(foo) -> foo_mod::implement,
///     function -> bar_mod::bar, // uses `bar` as the macro name
///     
///     // Using an already imported module with the `use` keyword
///     function(baz) -> use existing_mod::function_impl,
///     
///     // Explicitly declaring a module with the `mod` keyword
///     function(fizz) -> mod explicit_mod::function_impl,
///     
///     function(buzz) -> "src/impls/hello.rs"::hello,
///     // impl in custom crate path, uses 'world' as the macro name
///     function -> @"custom_src/impls/world.rs"::world,  
///     
///     attribute(my_attr) -> attrs::process,
///     attribute(use_attr) -> use imported_attr_mod::process,
///     attribute -> attrs::custom,  // uses 'custom' as the macro name
///     
///     derive(MyDerive) -> derives::generate,
///     derive(ImportedDerive) -> use imported_derive_mod::generate,
///     derive(NodeTypeChecks, attributes(node_category))
///         -> derive_impl_with_attrs::impl_with_attributes // derive with helper attributes
/// );
/// ```
#[macro_export]
macro_rules! macros {
    () => {};

    // function patterns with trailing comma
    (function -> $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($func -> $module::$func);
        $crate::macros!($($tail)*);
    };
    (function -> $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($func -> $func);
        $crate::macros!($($tail)*);
    };
    (function -> @$path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($func -> @$path::$func);
        $crate::macros!($($tail)*);
    };
    (function($name:ident) -> $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($name -> $func);
        $crate::macros!($($tail)*);
    };
    (function($name:ident) -> $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($name -> $module::$func);
        $crate::macros!($($tail)*);
    };
    (function($name:ident) -> @$path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($name -> @$path::$func);
        $crate::macros!($($tail)*);
    };
    (function($name:ident) -> use $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($name -> use $func);
        $crate::macros!($($tail)*);
    };
    (function($name:ident) -> use $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($name -> use $module::$func);
        $crate::macros!($($tail)*);
    };
    (function($name:ident) -> mod $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($name -> mod $func);
        $crate::macros!($($tail)*);
    };
    (function($name:ident) -> mod $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($name -> mod $module::$func);
        $crate::macros!($($tail)*);
    };
    (function($name:ident) -> $path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::proc_macro!($name -> $path::$func);
        $crate::macros!($($tail)*);
    };

    // attribute patterns with trailing comma
    (attribute -> $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($func -> $func);
        $crate::macros!($($tail)*);
    };
    (attribute -> $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($func -> $module::$func);
        $crate::macros!($($tail)*);
    };
    (attribute -> @$path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($func -> @$path::$func);
        $crate::macros!($($tail)*);
    };
    (attribute($name:ident) -> $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($name -> $func);
        $crate::macros!($($tail)*);
    };
    (attribute($name:ident) -> $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($name -> $module::$func);
        $crate::macros!($($tail)*);
    };
    (attribute($name:ident) -> @$path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($name -> @$path::$func);
        $crate::macros!($($tail)*);
    };

    // attribute use patterns with trailing comma
    (attribute($name:ident) -> use $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($name -> use $func);
        $crate::macros!($($tail)*);
    };
    (attribute($name:ident) -> use $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($name -> use $module::$func);
        $crate::macros!($($tail)*);
    };
    (attribute($name:ident) -> mod $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($name -> mod $func);
        $crate::macros!($($tail)*);
    };
    (attribute($name:ident) -> mod $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::attr_macro!($name -> mod $module::$func);
        $crate::macros!($($tail)*);
    };

    (derive($name:ident, attributes($($attr:ident),*)) -> $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name, attributes($($attr),*)) -> $module::$func);
        $crate::macros!($($tail)*);
    };

    (derive($name:ident, attributes($($attr:ident),*)) -> $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),*)) -> $module::$func);
    };

    (derive($name:ident, attributes($($attr:ident),+)) -> $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> $module::$func);
        $crate::macros!($($tail)*);
    };

    (derive($name:ident, attributes($($attr:ident),+)) -> use $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> use $module::$func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> mod $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> mod $module::$func);
        $crate::macros!($($tail)*);
    };

    // derive patterns with trailing comma
    (derive($name:ident) -> $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name) -> $func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident) -> $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name) -> $module::$func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident) -> @$path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name) -> @$path::$func);
        $crate::macros!($($tail)*);
    };

    (derive($name:ident) -> use $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name) -> use $func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident) -> use $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name) -> use $module::$func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident) -> mod $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name) -> mod $func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident) -> mod $module:ident :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name) -> mod $module::$func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> $func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> @$path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> @$path::$func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> use $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> use $func);
        $crate::macros!($($tail)*);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> mod $func:ident , $($tail:tt)*) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> mod $func);
        $crate::macros!($($tail)*);
    };

    // terminal patterns (without trailing comma)
    (function -> $func:ident) => {
        $crate::proc_macro!($func -> $func);
    };
    (function -> $module:ident :: $func:ident) => {
        $crate::proc_macro!($func -> $module::$func);
    };
    (function -> @$path:literal :: $func:ident) => {
        $crate::proc_macro!($func -> @$path::$func);
    };
    (function($name:ident) -> $func:ident) => {
        $crate::proc_macro!($name -> $func);
    };
    (function($name:ident) -> $module:ident :: $func:ident) => {
        $crate::proc_macro!($name -> $module::$func);
    };
    (function($name:ident) -> @$path:literal :: $func:ident) => {
        $crate::proc_macro!($name -> @$path::$func);
    };
    (function($name:ident) -> use $func:ident) => {
        $crate::proc_macro!($name -> use $func);
    };
    (function($name:ident) -> use $module:ident :: $func:ident) => {
        $crate::proc_macro!($name -> use $module::$func);
    };
    (function($name:ident) -> mod $func:ident) => {
        $crate::proc_macro!($name -> mod $func);
    };
    (function($name:ident) -> mod $module:ident :: $func:ident) => {
        $crate::proc_macro!($name -> mod $module::$func);
    };

    (attribute -> $func:ident) => {
        $crate::attr_macro!($func -> $func);
    };
    (attribute -> $module:ident :: $func:ident) => {
        $crate::attr_macro!($func -> $module::$func);
    };
    (attribute -> @$path:literal :: $func:ident) => {
        $crate::attr_macro!($func -> @$path::$func);
    };
    (attribute($name:ident) -> $func:ident) => {
        $crate::attr_macro!($name -> $func);
    };
    (attribute($name:ident) -> $module:ident :: $func:ident) => {
        $crate::attr_macro!($name -> $module::$func);
    };
    (attribute($name:ident) -> @$path:literal :: $func:ident) => {
        $crate::attr_macro!($name -> @$path::$func);
    };
    (attribute($name:ident) -> use $func:ident) => {
        $crate::attr_macro!($name -> use $func);
    };
    (attribute($name:ident) -> use $module:ident :: $func:ident) => {
        $crate::attr_macro!($name -> use $module::$func);
    };
    (attribute($name:ident) -> mod $func:ident) => {
        $crate::attr_macro!($name -> mod $func);
    };
    (attribute($name:ident) -> mod $module:ident :: $func:ident) => {
        $crate::attr_macro!($name -> mod $module::$func);
    };

    (derive($name:ident) -> $func:ident) => {
        $crate::derive_macro!(($name) -> $func);
    };
    (derive($name:ident) -> $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name) -> $module::$func);
    };
    (derive($name:ident) -> @$path:literal :: $func:ident) => {
        $crate::derive_macro!(($name) -> @$path::$func);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> $func);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> $module::$func);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> @$path:literal :: $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> @$path::$func);
    };
    (derive($name:ident) -> use $func:ident) => {
        $crate::derive_macro!(($name) -> use $func);
    };
    (derive($name:ident) -> use $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name) -> use $module::$func);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> use $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> use $func);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> use $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> use $module::$func);
    };
    (derive($name:ident) -> mod $func:ident) => {
        $crate::derive_macro!(($name) -> mod $func);
    };
    (derive($name:ident) -> mod $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name) -> mod $module::$func);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> mod $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> mod $func);
    };
    (derive($name:ident, attributes($($attr:ident),+)) -> mod $module:ident :: $func:ident) => {
        $crate::derive_macro!(($name, attributes($($attr),+)) -> mod $module::$func);
    };
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_integration_crate() {
        let status = Command::new("cargo")
            .args(["test", "-p", "integration_test"])
            .status()
            .expect("Failed to run integration tests");

        assert!(status.success(), "Integration tests failed");
    }
}
