//! `DefaultImpl` bounds every type parameter, including one no field uses.
//!
//! That is what the standard library's own `derive(Default)` does, and reproducing it is a
//! claim `derive_default_shapes.rs` can only test in the positive direction: an
//! implementation bounding only the parameters its fields use passes all seven of those.
//! This is the direction that separates them.

use examples::DefaultImpl;

#[derive(DefaultImpl)]
struct Phantom<T> {
    marker: core::marker::PhantomData<T>,
    count:  u8,
}

struct NotDefault;

fn main() {
    // `NotDefault` has no `Default`, and the parameter is used by nothing but a
    // `PhantomData`, so nothing in the value needs one. The bound is there anyway.
    let _ = Phantom::<NotDefault>::default();
}
