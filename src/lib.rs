#![no_std]

//! # okurrr
//!
//! `okerrr!` (and its aliases `okerr!`, `okurrr!`, `okurr!`) provides lightweight, ergonomic macros for Rust
//! to collapse multi-line `match` patterns on `Result` and `Option` into clean 1-liners while retaining full
//! control-flow (`return`, `break`, `continue`) and error payload bindings (`err => ...`).
//!
//! ## Why `okerrr!`?
//!
//! While Rust 1.65+ introduced `let Ok(val) = res else { return ... };`, standard `let-else` statements
//! **cannot bind the `Err(e)` payload** inside the `else` block. `unwrap_or_else` closures cannot perform
//! control flow (`return`, `break`, `continue`) out of the enclosing scope.
//!
//! `okerrr!` bridges this gap effortlessly by harmonizing `Ok` and `Err`:
//!
//! ```rust
//! use okurrr::okerrr;
//!
//! fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
//!     let val = okerrr!(input, err => {
//!         // 'err' payload is bound here, and we can still early-return!
//!         return Err(err);
//!     });
//!     Ok(val * 2)
//! }
//! ```
//!
//! ## OpenTelemetry / Cloud-Native Observability
//!
//! Enable the `tracing` or `otel` feature in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! okurrr = { version = "0.1", features = ["otel"] }
//! ```
//!
//! When the feature is active, error branches in `okerrr!` automatically emit a `tracing::error!` event!

/// `okerrr!` macro for ergonomic `Result<T, E>` pattern matching.
///
/// # Patterns
/// - `okerrr!(expr, err => handler)` : Binds error payload `err` into `handler`.
/// - `okerrr!(expr, else err => handler)` : Keyword variant of error payload binding.
/// - `okerrr!(expr, else fallback)` : Fallback value or block when `Err`.
/// - `okerrr!(expr, fallback)` : 2-argument fallback shorthand.
/// - `okerrr!(expr)` : Early returns `Err(From::from(err))`.
#[macro_export]
macro_rules! okerrr {
    // Pattern: okerrr!(expr, err => handler)
    ($expr:expr, $err:ident => $handler:expr) => {
        match $expr {
            ::core::result::Result::Ok(val) => val,
            ::core::result::Result::Err($err) => {
                #[cfg(feature = "tracing")]
                {
                    $crate::__log_err(&$err);
                }
                $handler
            }
        }
    };

    // Pattern: okerrr!(expr, else err => handler)
    ($expr:expr, else $err:ident => $handler:expr) => {
        $crate::okerrr!($expr, $err => $handler)
    };

    // Pattern: okerrr!(expr, else fallback)
    ($expr:expr, else $fallback:expr) => {
        match $expr {
            ::core::result::Result::Ok(val) => val,
            ::core::result::Result::Err(err) => {
                #[cfg(feature = "tracing")]
                {
                    $crate::__log_err(&err);
                }
                $fallback
            }
        }
    };

    // Pattern: okerrr!(expr, fallback)
    ($expr:expr, $fallback:expr) => {
        $crate::okerrr!($expr, else $fallback)
    };

    // Pattern: okerrr!(expr) -> returns early with Err(From::from(err))
    ($expr:expr) => {
        match $expr {
            ::core::result::Result::Ok(val) => val,
            ::core::result::Result::Err(err) => {
                #[cfg(feature = "tracing")]
                {
                    $crate::__log_err(&err);
                }
                return ::core::result::Result::Err(::core::convert::From::from(err));
            }
        }
    };
}

/// Macro alias for `okerrr!` with 2 r's.
#[macro_export]
macro_rules! okerr {
    ($($tok:tt)*) => {
        $crate::okerrr!($($tok)*)
    };
}

/// Macro alias for `okerrr!` for phonetics and backwards compatibility.
#[macro_export]
macro_rules! okurrr {
    ($($tok:tt)*) => {
        $crate::okerrr!($($tok)*)
    };
}

/// Macro alias for `okerrr!` for phonetics and backwards compatibility.
#[macro_export]
macro_rules! okurr {
    ($($tok:tt)*) => {
        $crate::okerrr!($($tok)*)
    };
}

/// `okerrr_some!` macro for `Option<T>` pattern matching (`Some`/`None`).
///
/// # Patterns
/// - `okerrr_some!(expr, else fallback)` : Evaluates fallback if `None`.
/// - `okerrr_some!(expr, fallback)` : 2-argument fallback shorthand.
/// - `okerrr_some!(expr)` : Early returns `None`.
#[macro_export]
macro_rules! okerrr_some {
    ($expr:expr, else $fallback:expr) => {
        match $expr {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => $fallback,
        }
    };

    ($expr:expr, $fallback:expr) => {
        $crate::okerrr_some!($expr, else $fallback)
    };

    ($expr:expr) => {
        match $expr {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => return ::core::option::Option::None,
        }
    };
}

/// Macro alias for `okerrr_some!` with 2 r's.
#[macro_export]
macro_rules! okerr_some {
    ($($tok:tt)*) => {
        $crate::okerrr_some!($($tok)*)
    };
}

/// Macro alias for `okerrr_some!` for phonetics and backwards compatibility.
#[macro_export]
macro_rules! okurrr_some {
    ($($tok:tt)*) => {
        $crate::okerrr_some!($($tok)*)
    };
}

/// Macro alias for `okerrr_some!` for phonetics and backwards compatibility.
#[macro_export]
macro_rules! okurr_some {
    ($($tok:tt)*) => {
        $crate::okerrr_some!($($tok)*)
    };
}

#[doc(hidden)]
#[cfg(feature = "tracing")]
pub fn __log_err<E: core::fmt::Display>(err: &E) {
    tracing::error!(target: "okerrr", error = %err, "okerrr caught error");
}

