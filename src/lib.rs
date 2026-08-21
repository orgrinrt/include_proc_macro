#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "README.md"))]
#![cfg_attr(feature = "no_std", no_std)]

// `no_std` here is the attribute and nothing else, and `no_alloc` is a statement rather than
// a change. This crate is `macro_rules!` and nothing but: it names no type, calls no
// function, and touches neither `std` nor an allocator at any point. What it expands *into*
// is a `#[proc_macro]` entry point, and a proc-macro crate cannot be `no_std` whatever this
// one does, because it runs on the host inside the compiler and syn, quote and proc-macro2
// all use std.
//
// So the features exist to be declarable by a consumer whose workspace turns them on
// everywhere, and to be checked rather than assumed: `tests/feature_matrix.rs` builds the
// crate under each and asserts the macros still expand.
//
// `no_alloc` is an alias for `no_std` and nothing in this crate reads it. No test
// distinguishes the two, because there is nothing to distinguish: nothing here
// allocates, so there is nothing for it to switch off.

// The three kinds of procedural macro differ in exactly three ways: the attribute
// they carry, the arguments they take, and how the implementation is called. Every
// other thing about them, and the whole of the path grammar in particular, is the
// same for all three.
//
// This file is arranged so that the shared part is written once. The path grammar
// lives in `__ipm_resolve`, and each kind supplies a small emitter saying what an
// item of that kind looks like. A form added to the resolver therefore exists for
// all three kinds by construction, rather than because someone remembered to write
// it out three times.
//
// It did not used to be arranged that way, and the parity broke exactly where you
// would expect. `proc_macro!(name -> function)`, the first pattern in its own
// documentation, expanded to an arm that had never existed, so it failed for every
// caller who tried it, while `attr_macro!` and `derive_macro!` spelled the same
// shorthand correctly. Separately, all three documented nested modules and none of
// them could parse the form: the matcher read `$($module:ident)::+ :: $func:ident`,
// where the repetition and the trailing segment are idents separated by the same
// token, so nothing says where the repetition stops and rustc reports a local
// ambiguity. Absorbing the function into the repetition removes the choice, because
// the last segment is then the function by construction.
//
// `arm_matrix_test/` asserts every cell of the grammar against every kind, so a form
// that is documented and does not work is a failing test rather than a bug report.

// ---------------------------------------------------------------------------
// Emitters. The only place the three kinds differ.
// ---------------------------------------------------------------------------

/// Emits a function-like procedural macro. Internal.
#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_emit_function {
    ($name:ident, [$($meta:tt)*], {$($prelude:tt)*}, $($path:tt)*) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($prelude)*
            $($path)*(input)
        }
    };
}

/// Emits an attribute procedural macro. Internal.
#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_emit_attribute {
    ($name:ident, [$($meta:tt)*], {$($prelude:tt)*}, $($path:tt)*) => {
        #[proc_macro_attribute]
        pub fn $name(
            attr: proc_macro::TokenStream,
            item: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            $($prelude)*
            $($path)*(attr, item)
        }
    };
}

/// Emits a derive procedural macro. Internal.
#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_emit_derive {
    ($name:ident, [$($meta:tt)*], {$($prelude:tt)*}, $($path:tt)*) => {
        #[allow(non_snake_case)]
        #[proc_macro_derive($name $($meta)*)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($prelude)*
            $($path)*(input)
        }
    };
}

// ---------------------------------------------------------------------------
// The path grammar. Written once, used by all three kinds.
// ---------------------------------------------------------------------------

/// Reads an implementation path and hands the resolved call to an emitter. Internal.
///
/// `use` and `mod` are matched before the bare forms deliberately. An `ident`
/// fragment matches keywords, so `use foo::bar` would otherwise be read as a
/// three-segment path whose first segment is called `use`.
#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_resolve {
    // Already in scope.
    ($emit:path, $name:ident, [$($meta:tt)*], use $func:ident) => {
        $emit!($name, [$($meta)*], {}, $func);
    };

    // Already imported, at any depth. One arm covers `use m::f` and `use a::b::c::f`.
    ($emit:path, $name:ident, [$($meta:tt)*], use $first:ident $(:: $seg:ident)+) => {
        $emit!($name, [$($meta)*], {}, $first $(:: $seg)+);
    };

    // Declare the root module, then resolve against it.
    ($emit:path, $name:ident, [$($meta:tt)*], mod $first:ident $(:: $seg:ident)+) => {
        mod $first;
        $crate::__ipm_resolve!($emit, $name, [$($meta)*], use $first $(:: $seg)+);
    };

    // `mod f` names a module and no function inside it. Say so, rather than letting
    // the matcher report that no rule applied.
    ($emit:path, $name:ident, [$($meta:tt)*], mod $func:ident) => {
        compile_error!(concat!(
            "include_proc_macro: `mod ",
            stringify!($func),
            "` names a module but no function inside it. Write `mod ",
            stringify!($func),
            "::the_function`, or drop the `mod` to call a function already in scope."
        ));
    };

    // A file, relative to the file holding the invocation.
    ($emit:path, $name:ident, [$($meta:tt)*], $path:literal :: $func:ident) => {
        $emit!($name, [$($meta)*], {
            #[path = $path]
            mod __inner;
        }, __inner::$func);
    };

    // A file, relative to the crate root.
    ($emit:path, $name:ident, [$($meta:tt)*], @$path:literal :: $func:ident) => {
        $emit!($name, [$($meta)*], {
            mod __inner {
                include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
            }
        }, __inner::$func);
    };

    // A path rooted at `crate` or `self` already names something reachable,
    // so there is nothing to declare. Without these two the general arm below
    // would try to emit `mod crate;`, which is not a thing. An `ident` fragment
    // matches keywords, so they have to be caught before it rather than after.
    // `super` is deliberately absent: a procedural macro item must sit at the crate
    // root, which has no parent, so no invocation could reach it.
    ($emit:path, $name:ident, [$($meta:tt)*], crate $(:: $seg:ident)+) => {
        $emit!($name, [$($meta)*], {}, crate $(:: $seg)+);
    };
    ($emit:path, $name:ident, [$($meta:tt)*], self $(:: $seg:ident)+) => {
        $emit!($name, [$($meta)*], {}, self $(:: $seg)+);
    };

    // A path with no keyword: declare the root module and resolve against it.
    ($emit:path, $name:ident, [$($meta:tt)*], $first:ident $(:: $seg:ident)+) => {
        $crate::__ipm_resolve!($emit, $name, [$($meta)*], mod $first $(:: $seg)+);
    };

    // A bare name: a function already in scope.
    ($emit:path, $name:ident, [$($meta:tt)*], $func:ident) => {
        $crate::__ipm_resolve!($emit, $name, [$($meta)*], use $func);
    };

    // Anything else, named rather than left to the matcher's own diagnostic.
    ($emit:path, $name:ident, [$($meta:tt)*], $($bad:tt)*) => {
        compile_error!(concat!(
            "include_proc_macro: cannot read the implementation path `",
            stringify!($($bad)*),
            "`. The forms are `f`, `use f`, `m::f`, `mod m::f`, `use m::f`, ",
            "`a::b::c::f`, `\"path/to/file.rs\"::f` and `@\"path/from/crate/root.rs\"::f`."
        ));
    };
}

/// Resolves a path whose last segment also names the macro. Internal.
///
/// Every form ends in the implementation function's own identifier, so the last
/// token is the name to use. Reaching it takes a walk: a repetition cannot be
/// followed by a capture, because nothing would say where the repetition stops.
#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_infer_name {
    // One token left and it is the function, so that is the name.
    ($emit:path, [$($meta:tt)*], [$($seen:tt)*], $func:ident) => {
        $crate::__ipm_resolve!($emit, $func, [$($meta)*], $($seen)* $func);
    };
    // More to come: keep this token and walk on.
    ($emit:path, [$($meta:tt)*], [$($seen:tt)*], $head:tt $($tail:tt)+) => {
        $crate::__ipm_infer_name!($emit, [$($meta)*], [$($seen)* $head], $($tail)+);
    };
}

// ---------------------------------------------------------------------------
// The three public single-macro forms.
// ---------------------------------------------------------------------------

/// Defines a function-like procedural macro whose implementation lives elsewhere.
///
/// The implementation is an ordinary function taking and returning a
/// `proc_macro::TokenStream`. It carries no `#[proc_macro]` attribute, because it
/// cannot: that attribute is only legal on a public item at the root of a
/// proc-macro crate, which is the restriction this crate exists to work around.
///
/// ## Where the implementation can be
///
/// | Form | Means |
/// |---|---|
/// | `name -> f` | `f` is already in scope |
/// | `name -> use f` | the same, said explicitly |
/// | `name -> m::f` | declare `mod m`, call `m::f` |
/// | `name -> mod m::f` | the same, said explicitly |
/// | `name -> use m::f` | `m` is already declared |
/// | `name -> a::b::c::f` | declare `mod a`, call `a::b::c::f` |
/// | `name -> use a::b::c::f` | `a` is already declared |
/// | `name -> "path/to/file.rs"::f` | a file, relative to this one |
/// | `name -> @"path/from/crate/root.rs"::f` | a file, relative to the crate root |
///
/// Every row is asserted in `arm_matrix_test/`, for this macro and for the other two.
///
/// See also [`attr_macro!`](crate::attr_macro), [`derive_macro!`](crate::derive_macro),
/// and [`macros!`](crate::macros), which declares any number of them at once.
#[macro_export]
macro_rules! proc_macro {
    ($name:ident -> $($spec:tt)+) => {
        $crate::__ipm_resolve!($crate::__ipm_emit_function, $name, [], $($spec)+);
    };
}

/// Defines an attribute procedural macro whose implementation lives elsewhere.
///
/// The implementation takes two `proc_macro::TokenStream`s, the attribute's own
/// arguments and the item it is applied to, and returns the replacement item.
///
/// The implementation path takes the same forms as
/// [`proc_macro!`](crate::proc_macro); the grammar is shared rather than
/// reimplemented, which is what keeps the two from disagreeing.
#[macro_export]
macro_rules! attr_macro {
    ($name:ident -> $($spec:tt)+) => {
        $crate::__ipm_resolve!($crate::__ipm_emit_attribute, $name, [], $($spec)+);
    };
}

/// Defines a derive procedural macro whose implementation lives elsewhere.
///
/// The name is parenthesised because a derive may also declare helper attributes:
/// `derive_macro!((Validate, attributes(required, range)) -> validators::check)`.
///
/// The implementation path takes the same forms as
/// [`proc_macro!`](crate::proc_macro).
#[macro_export]
macro_rules! derive_macro {
    (($name:ident, attributes($($attr:ident),* $(,)?)) -> $($spec:tt)+) => {
        $crate::__ipm_resolve!(
            $crate::__ipm_emit_derive, $name, [, attributes($($attr),*)], $($spec)+
        );
    };
    (($name:ident) -> $($spec:tt)+) => {
        $crate::__ipm_resolve!($crate::__ipm_emit_derive, $name, [], $($spec)+);
    };
}

// ---------------------------------------------------------------------------
// The list form.
// ---------------------------------------------------------------------------

/// Declares any number of procedural macros in one place.
///
/// Entries are separated by commas, and a trailing comma is allowed. Each entry
/// names a kind, optionally a macro name, and an implementation path:
///
/// ```rust,ignore
/// include_proc_macro::macros!(
///     function(greet) -> greetings::hello,
///     function -> parsers::parse,               // the macro is called `parse`
///     attribute(instrument) -> use tracing_impl::instrument,
///     attribute(cached) -> @"src/impls/cache.rs"::cached,
///     derive(Default) -> derives::default_impl,
///     derive(Validate, attributes(required, range)) -> validators::check,
/// );
/// ```
///
/// Leaving the name out takes it from the last segment of the path, so
/// `function -> parsers::parse` declares a macro called `parse`. A derive always
/// names itself, because the name is what the deriving type writes.
///
/// The implementation path takes the same forms as
/// [`proc_macro!`](crate::proc_macro), file paths included. The grammar is shared
/// with the single-macro forms rather than enumerated a second time here.
#[macro_export]
macro_rules! macros {
    ($($entries:tt)*) => {
        $crate::__ipm_split!($($entries)* ,);
    };
}

/// Splits a comma-separated list of declarations. Internal.
///
/// A comma inside `attributes(a, b)` sits inside a parenthesised token tree, which
/// is one token tree, so it is never mistaken for a separator.
///
/// Each arm consumes one whole entry, and the recursion is therefore one level per
/// declaration. That is the reason for the shape. A splitter that munched a token
/// at a time reached the default recursion limit of 128 at about twenty entries,
/// because `::` is two token trees and an entry is roughly ten; separating the
/// head from the path cost a second level and capped it near sixty. Declaring many
/// macros in one crate is what this crate is for, so the limit sat across the
/// intended use rather than beyond it.
///
/// The arms differ only in where a path ends. What a path means is decided once,
/// by `__ipm_resolve`, which is what keeps the list form and the single-macro
/// forms from drifting apart.
#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_split {
    () => {};
    // A leading or repeated comma. Tolerated rather than diagnosed; the trailing
    // comma this macro is always called with makes the repeated case ordinary.
    (, $($tail:tt)*) => { $crate::__ipm_split!($($tail)*); };

    ($kind:ident $args:tt -> use $first:ident $(:: $seg:ident)* , $($tail:tt)*) => {
        $crate::__ipm_one!($kind $args -> use $first $(:: $seg)*);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident -> use $first:ident $(:: $seg:ident)* , $($tail:tt)*) => {
        $crate::__ipm_one!($kind -> use $first $(:: $seg)*);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident $args:tt -> mod $first:ident $(:: $seg:ident)* , $($tail:tt)*) => {
        $crate::__ipm_one!($kind $args -> mod $first $(:: $seg)*);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident -> mod $first:ident $(:: $seg:ident)* , $($tail:tt)*) => {
        $crate::__ipm_one!($kind -> mod $first $(:: $seg)*);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident $args:tt -> @$path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::__ipm_one!($kind $args -> @$path :: $func);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident -> @$path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::__ipm_one!($kind -> @$path :: $func);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident $args:tt -> $path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::__ipm_one!($kind $args -> $path :: $func);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident -> $path:literal :: $func:ident , $($tail:tt)*) => {
        $crate::__ipm_one!($kind -> $path :: $func);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident $args:tt -> $first:ident $(:: $seg:ident)* , $($tail:tt)*) => {
        $crate::__ipm_one!($kind $args -> $first $(:: $seg)*);
        $crate::__ipm_split!($($tail)*);
    };

    ($kind:ident -> $first:ident $(:: $seg:ident)* , $($tail:tt)*) => {
        $crate::__ipm_one!($kind -> $first $(:: $seg)*);
        $crate::__ipm_split!($($tail)*);
    };

    ($($bad:tt)*) => {
        compile_error!(concat!(
            "include_proc_macro: cannot read the declaration `",
            stringify!($($bad)*),
            "`. An entry is `function(name) -> path`, `function -> path`, ",
            "`attribute(name) -> path`, `attribute -> path`, `derive(Name) -> path`, ",
            "or `derive(Name, attributes(a, b)) -> path`, where a path is one of ",
            "`f`, `use f`, `m::f`, `mod m::f`, `use m::f`, `a::b::c::f`, ",
            "`\"path/to/file.rs\"::f` or `@\"path/from/crate/root.rs\"::f`."
        ));
    };
}

/// Reports a declaration whose macro would call itself. Internal.
#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_self_reference {
    ($kind:ident, $func:ident) => {
        compile_error!(concat!(
            "include_proc_macro: `",
            stringify!($kind),
            " -> ",
            stringify!($func),
            "` would take its name from `",
            stringify!($func),
            "`, and the generated item would then shadow the function it is supposed \
             to call, so the macro would call itself. Give the macro its own name with \
             `",
            stringify!($kind),
            "(some_name) -> ",
            stringify!($func),
            "`, or move the implementation into a module and write `",
            stringify!($kind),
            " -> some_module::",
            stringify!($func),
            "`."
        ));
    };
}

/// Declares one entry from the list form. Internal.
#[doc(hidden)]
#[macro_export]
macro_rules! __ipm_one {
    (function($name:ident) -> $($spec:tt)+) => {
        $crate::__ipm_resolve!($crate::__ipm_emit_function, $name, [], $($spec)+);
    };

    // Inferring the name from a bare function is degenerate: the generated item
    // takes that same name in that same scope, so the call in its body resolves to
    // itself. Refuse it, rather than emitting a macro that recurses until the
    // compiler dies. Caught by the arm matrix, where it crashed rustc with SIGBUS
    // after eighty-four frames.
    (function -> $func:ident) => {
        $crate::__ipm_self_reference!(function, $func);
    };
    (function -> use $func:ident) => {
        $crate::__ipm_self_reference!(function, $func);
    };
    (function -> $($spec:tt)+) => {
        $crate::__ipm_infer_name!($crate::__ipm_emit_function, [], [], $($spec)+);
    };

    (attribute($name:ident) -> $($spec:tt)+) => {
        $crate::__ipm_resolve!($crate::__ipm_emit_attribute, $name, [], $($spec)+);
    };
    (attribute -> $func:ident) => {
        $crate::__ipm_self_reference!(attribute, $func);
    };
    (attribute -> use $func:ident) => {
        $crate::__ipm_self_reference!(attribute, $func);
    };
    (attribute -> $($spec:tt)+) => {
        $crate::__ipm_infer_name!($crate::__ipm_emit_attribute, [], [], $($spec)+);
    };

    (derive($name:ident, attributes($($attr:ident),* $(,)?)) -> $($spec:tt)+) => {
        $crate::__ipm_resolve!(
            $crate::__ipm_emit_derive, $name, [, attributes($($attr),*)], $($spec)+
        );
    };
    (derive($name:ident) -> $($spec:tt)+) => {
        $crate::__ipm_resolve!($crate::__ipm_emit_derive, $name, [], $($spec)+);
    };

    ($($bad:tt)*) => {
        compile_error!(concat!(
            "include_proc_macro: cannot read the declaration `",
            stringify!($($bad)*),
            "`. An entry is `function(name) -> path`, `function -> path`, ",
            "`attribute(name) -> path`, `attribute -> path`, `derive(Name) -> path`, ",
            "or `derive(Name, attributes(a, b)) -> path`."
        ));
    };
}
