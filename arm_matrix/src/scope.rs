pub fn f_bare_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn f_bare_mark() -> &'static str { \"f_bare\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn f_use_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn f_use_mark() -> &'static str { \"f_use\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn a_bare_fn(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn a_bare_mark() -> &'static str { \"a_bare\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn a_use_fn(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn a_use_mark() -> &'static str { \"a_use\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn d_bare_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn d_bare_mark() -> &'static str { \"d_bare\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn d_use_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn d_use_mark() -> &'static str { \"d_use\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn da_bare_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn da_bare_mark() -> &'static str { \"da_bare\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn da_use_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn da_use_mark() -> &'static str { \"da_use\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn lf_bare_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn lf_bare_mark() -> &'static str { \"lf_bare\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn lf_use_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn lf_use_mark() -> &'static str { \"lf_use\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn la_bare_fn(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn la_bare_mark() -> &'static str { \"la_bare\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn la_use_fn(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn la_use_mark() -> &'static str { \"la_use\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn ld_bare_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn ld_bare_mark() -> &'static str { \"ld_bare\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn ld_use_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn ld_use_mark() -> &'static str { \"ld_use\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn lda_bare_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn lda_bare_mark() -> &'static str { \"lda_bare\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
pub fn lda_use_fn(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn lda_use_mark() -> &'static str { \"lda_use\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
