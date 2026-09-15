#![no_std]

//! # okurrr
//!
//! `okurrr` (and its alias `okurr!`) provides lightweight, ergonomic macros for Rust to collapse
//! multi-line `match` patterns on `Result` and `Option` into clean 1-liners while retaining full
//! control-flow (`return`, `break`, `continue`) and error payload bindings (`err => ...`).
//!
//! ## Why `okurrr!`?
//!
//! While Rust 1.65+ introduced `let Ok(val) = res else { return ... };`, standard `let-else` statements
//! **cannot bind the `Err(e)` payload** inside the `else` block. `unwrap_or_else` closures cannot perform
//! control flow (`return`, `break`, `continue`) out of the enclosing scope.
//!
//! `okurrr!` bridges this gap effortlessly:
//!
//! ```rust
//! use okurrr::okurrr;
//!
//! fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
//!     let val = okurrr!(input, err => {
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
//! When the feature is active, error branches in `okurrr!` automatically emit a `tracing::error!` event!

/// `okurrr!` macro for ergonomic `Result<T, E>` pattern matching.
///
/// # Patterns
/// - `okurrr!(expr, err => handler)` : Binds error payload `err` into `handler`.
/// - `okurrr!(expr, else err => handler)` : Keyword variant of error payload binding.
/// - `okurrr!(expr, else fallback)` : Fallback value or block when `Err`.
/// - `okurrr!(expr, fallback)` : 2-argument fallback shorthand.
/// - `okurrr!(expr)` : Early returns `Err(From::from(err))`.
#[macro_export]
macro_rules! okurrr {
    // Pattern: okurrr!(expr, err => handler)
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

    // Pattern: okurrr!(expr, else err => handler)
    ($expr:expr, else $err:ident => $handler:expr) => {
        $crate::okurrr!($expr, $err => $handler)
    };

    // Pattern: okurrr!(expr, else fallback)
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

    // Pattern: okurrr!(expr, fallback)
    ($expr:expr, $fallback:expr) => {
        $crate::okurrr!($expr, else $fallback)
    };

    // Pattern: okurrr!(expr) -> returns early with Err(From::from(err))
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

/// Macro alias for `okurrr!` with 2 r's for backwards compatibility.
#[macro_export]
macro_rules! okurr {
    ($($tok:tt)*) => {
        $crate::okurrr!($($tok)*)
    };
}

/// `okurrr_some!` macro for `Option<T>` pattern matching (`Some`/`None`).
///
/// # Patterns
/// - `okurrr_some!(expr, else fallback)` : Evaluates fallback if `None`.
/// - `okurrr_some!(expr, fallback)` : 2-argument fallback shorthand.
/// - `okurrr_some!(expr)` : Early returns `None`.
#[macro_export]
macro_rules! okurrr_some {
    ($expr:expr, else $fallback:expr) => {
        match $expr {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => $fallback,
        }
    };

    ($expr:expr, $fallback:expr) => {
        $crate::okurrr_some!($expr, else $fallback)
    };

    ($expr:expr) => {
        match $expr {
            ::core::option::Option::Some(val) => val,
            ::core::option::Option::None => return ::core::option::Option::None,
        }
    };
}

/// Macro alias for `okurrr_some!` with 2 r's.
#[macro_export]
macro_rules! okurr_some {
    ($($tok:tt)*) => {
        $crate::okurrr_some!($($tok)*)
    };
}

#[doc(hidden)]
#[cfg(feature = "tracing")]
pub fn __log_err<E: core::fmt::Display>(err: &E) {
    tracing::error!(target: "okurrr", error = %err, "okurrr caught error");
}
