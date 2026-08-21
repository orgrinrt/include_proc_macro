//! `DefaultImpl` refuses an enum.
//!
//! There is no sensible default for one without being told which variant, so the macro
//! declines rather than guessing. A refusal that nothing pins can be deleted by accident,
//! and this is what pins it.

use examples::DefaultImpl;

#[derive(DefaultImpl)]
enum Mode {
    Fast,
    Slow,
}

fn main() {
    let _ = Mode::Fast;
}
