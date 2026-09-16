#![no_std]

//! # okerrr
//!
//! A `no_std` declarative macro for binding a `Result::Err` payload in a
//! diverging `else` branch.
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
//! [`okerrr!`] keeps that behavior at the call site with less repeated
//! structure:
//!
//! ```rust
//! use okerrr::okerrr;
//!
//! fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
//!     let value = okerrr!(input, else error => return Err(error));
//!     Ok(value * 2)
//! }
//! ```
//!
//! This is a bound `else`: the `Ok` payload continues in the surrounding
//! scope, while the `else` branch binds the `Err` payload and must diverge.
//! Its handler can `return`, `break`, `continue`, panic, loop forever, or call
//! another never-returning expression.
//!
//! Unlike a closure fallback, those control-flow expressions act on the
//! surrounding function or loop. The input is evaluated exactly once.
//!
//! An expression containing `.await` works when the invocation is already in
//! an async context; the macro does not await implicitly.
//!
//! ## Caller-side instrumentation
//!
//! The macro does not log or require an error formatting trait. An error
//! handler can call `tracing::error!` when that caller chooses to record the
//! error. Subscriber and OpenTelemetry export setup belong to the application.

/// Extracts an `Ok` payload or runs a diverging handler bound to the error.
///
/// The only supported form is `okerrr!(expr, else error => handler)`. The input
/// expression is evaluated once. On `Ok`, the macro evaluates to its payload.
/// On `Err`, the payload is moved into `error` and `handler` must diverge.
///
/// A handler may `return`, `break`, `continue`, panic, loop forever, or invoke
/// another expression that never returns. Because the handler is expanded at
/// the call site rather than inside a closure, its control flow applies to the
/// surrounding function or loop.
///
/// The macro does not log, await, convert errors, or require `Debug` or
/// `Display`. Those decisions remain in the handler.
///
/// # Async input
///
/// ```rust
/// # use okerrr::okerrr;
/// async fn load(input: Result<u32, &'static str>) -> Result<u32, &'static str> {
///     let value = okerrr!(async { input }.await, else error => return Err(error));
///     Ok(value)
/// }
/// ```
///
/// # Non-diverging handlers are rejected
///
/// ```compile_fail
/// # use okerrr::okerrr;
/// fn recover(input: Result<u32, &'static str>) -> u32 {
///     okerrr!(input, else error => {
///         let _ = error;
///         0
///     })
/// }
/// ```
#[macro_export]
macro_rules! okerrr {
    ($expr:expr, else $error:ident => $handler:expr) => {{
        match $expr {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err($error) => {
                #[allow(unreachable_code, clippy::diverging_sub_expression)]
                let __okerrr_never: ::core::convert::Infallible = $handler;
                #[allow(unreachable_code, unused_variables)]
                match __okerrr_never {}
            }
        }
    }};
}
