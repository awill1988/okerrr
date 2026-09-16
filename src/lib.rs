#![no_std]

//! # okerrr
//!
//! Declarative macros for handling `Result` and `Option` without repeating a
//! `match` at every call site. Use [`okerrr!`] and [`okerrr_some!`] in new code;
//! [`okerr!`] and [`okerr_some!`] are convenience spellings of the same macros.
//!
//! ## The pattern
//!
//! A `match` lets an error handler bind its payload and return from the caller:
//!
//! ```rust
//! fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
//!     let value = match input {
//!         Ok(value) => value,
//!         Err(error) => return Err(error),
//!     };
//!     Ok(value * 2)
//! }
//! ```
//!
//! `okerrr!` keeps that behavior at the call site with less repeated structure:
//!
//! ```rust
//! use okerrr::okerrr;
//!
//! fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
//!     let value = okerrr!(input, error => return Err(error));
//!     Ok(value * 2)
//! }
//! ```
//!
//! `if let` remains useful when only one branch matters. Here, binding the
//! `Err` payload and using `return`, `break`, or `continue` in the caller are
//! the reasons to use the macro instead of a closure-based fallback.
//!
//! ## Caller-side instrumentation
//!
//! The macro does not log or require an error formatting trait. An error
//! handler can call `tracing::error!` when that caller chooses to record the
//! error. Subscriber and OpenTelemetry export setup belong to the application.

/// Canonical macro for handling `Result<T, E>` at the call site.
///
/// The input expression is evaluated once. A fallback runs only for `Err`.
/// Unlike a closure fallback, a handler can `return`, `break`, or `continue`
/// from the surrounding function or loop.
///
/// # Forms
///
/// - `okerrr!(expr, error => handler)` binds the error payload.
/// - `okerrr!(expr, else error => handler)` is the keyword variant.
/// - `okerrr!(expr, else fallback)` supplies a fallback value or block.
/// - `okerrr!(expr, fallback)` is the compact fallback spelling.
/// - `okerrr!(expr)` returns `Err(From::from(error))` from the caller.
///
/// The macro does not emit an event or require a formatting trait. Callers
/// can use `tracing::error!` inside a bound handler when an error merits
/// recording.
#[macro_export]
macro_rules! okerrr {
    ($expr:expr, $err:ident => $handler:expr) => {
        match $expr {
            ::core::result::Result::Ok(val) => val,
            ::core::result::Result::Err($err) => $handler,
        }
    };

    ($expr:expr, else $err:ident => $handler:expr) => {
        $crate::okerrr!($expr, $err => $handler)
    };

    ($expr:expr, else $fallback:expr) => {
        match $expr {
            ::core::result::Result::Ok(val) => val,
            ::core::result::Result::Err(_) => $fallback,
        }
    };

    ($expr:expr, $fallback:expr) => {
        $crate::okerrr!($expr, else $fallback)
    };

    ($expr:expr) => {
        match $expr {
            ::core::result::Result::Ok(val) => val,
            ::core::result::Result::Err(err) => {
                return ::core::result::Result::Err(::core::convert::From::from(err));
            }
        }
    };
}

/// Convenience alias for [`okerrr!`]. Prefer `okerrr!` in examples and new code.
#[macro_export]
macro_rules! okerr {
    ($($tok:tt)*) => {
        $crate::okerrr!($($tok)*)
    };
}

/// Canonical macro for handling `Option<T>` at the call site.
///
/// The input expression is evaluated once. A fallback runs only for `None`.
///
/// # Forms
///
/// - `okerrr_some!(expr, else fallback)` evaluates a fallback on `None`.
/// - `okerrr_some!(expr, fallback)` is the compact fallback spelling.
/// - `okerrr_some!(expr)` returns `None` from the caller.
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

/// Convenience alias for [`okerrr_some!`]. Prefer the canonical spelling in
/// examples and new code.
#[macro_export]
macro_rules! okerr_some {
    ($($tok:tt)*) => {
        $crate::okerrr_some!($($tok)*)
    };
}
