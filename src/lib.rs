#![no_std]

//! # okerrr
//!
//! A `no_std` declarative macro for dispatching a `Result::Err` payload through
//! diverging `case` clauses.
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
//!     let value = okerrr!(input, case error => return Err(error));
//!     Ok(value * 2)
//! }
//! ```
//!
//! The `Ok` payload continues in the surrounding scope. Each `case` pattern
//! matches the raw `Err` payload, and every handler must diverge. A handler can
//! `return`, `break`, `continue`, panic, loop forever, or call another
//! never-returning expression.
//!
//! The clause-oriented style takes inspiration from
//! [Elixir's `case` control flow][elixir-case]. Patterns, `if` guards,
//! exhaustiveness, ownership, and divergence keep their Rust semantics. Unlike
//! a closure fallback, control-flow expressions act on the surrounding
//! function or loop. The input is evaluated exactly once.
//!
//! [elixir-case]: https://hexdocs.pm/elixir/case-cond-and-if.html#case
//!
//! An expression containing `.await` works when the invocation is already in
//! an async context; the macro does not await implicitly.
//!
//! ## Caller-side instrumentation
//!
//! The macro does not log or require an error formatting trait. An error
//! handler can call `tracing::error!` when that caller chooses to record the
//! error. Subscriber and OpenTelemetry export setup belong to the application.

/// Extracts an `Ok` payload or dispatches the raw error through diverging cases.
///
/// The supported form is
/// `okerrr!(expr, case pattern if guard => handler, ...)`. The `if` guard is
/// optional. The input expression is evaluated once. On `Ok`, the macro
/// evaluates to its payload. On `Err`, the clauses exhaustively match the raw
/// error payload and every handler must diverge.
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
///     let value = okerrr!(async { input }.await, case error => return Err(error));
///     Ok(value)
/// }
/// ```
///
/// # Multiple error cases
///
/// ```rust
/// # use okerrr::okerrr;
/// enum FetchError {
///     Busy(u8),
///     Fatal,
/// }
///
/// fn load(input: Result<u32, FetchError>, retries: u8) -> Result<u32, FetchError> {
///     let value = okerrr!(
///         input,
///         case FetchError::Busy(attempt) if attempt < retries => return Ok(attempt.into()),
///         case error @ FetchError::Busy(_) => return Err(error),
///         case error => return Err(error),
///     );
///     Ok(value)
/// }
/// ```
///
/// # Non-diverging handlers are rejected
///
/// ```compile_fail
/// # use okerrr::okerrr;
/// fn recover(input: Result<u32, &'static str>) -> u32 {
///     okerrr!(input, case error => {
///         let _ = error;
///         0
///     })
/// }
/// ```
///
/// # Error cases must be exhaustive
///
/// ```compile_fail
/// # use okerrr::okerrr;
/// enum Error {
///     Missing,
///     Denied,
/// }
///
/// fn load(input: Result<u32, Error>) {
///     let _ = okerrr!(input, case Error::Missing => return);
/// }
/// ```
#[macro_export]
macro_rules! okerrr {
    (
        $expr:expr,
        $(case $error:pat $(if $guard:expr)? => $handler:expr),+ $(,)?
    ) => {{
        match $expr {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(__okerrr_error) => match __okerrr_error {
                $(
                    $error $(if $guard)? => {
                        #[allow(unreachable_code, clippy::diverging_sub_expression)]
                        let __okerrr_never: ::core::convert::Infallible = $handler;
                        #[allow(unreachable_code, unused_variables)]
                        match __okerrr_never {}
                    }
                ),+
            },
        }
    }};
}
