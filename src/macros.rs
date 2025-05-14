//! # === macros.rs ===
//!
//! adds several utility macros to the project
//! like some to extend readability like [`macro@not`]

/// add readability to logical negate
#[macro_export]
macro_rules! not {
    ($x:expr)  => { !$x };
}
