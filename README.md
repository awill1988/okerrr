# okerrr!

Small declarative macros for the `Result` and `Option` patterns that turn up at
every call site. The triple `r` is for a little fun; the API stays simple.

[![Crates.io](https://img.shields.io/crates/v/okerrr.svg)](https://crates.io/crates/okerrr)
[![Documentation](https://docs.rs/okerrr/badge.svg)](https://docs.rs/okerrr)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Why a macro?

When an error needs its payload and must return from the caller, a `match` is
often the right tool:

```rust
fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
    let value = match input {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    Ok(value * 2)
}
```

`okerrr!` keeps the same control flow and error binding while removing the
repeated pattern:

```rust
use okerrr::okerrr;

fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
    let value = okerrr!(input, error => return Err(error));
    Ok(value * 2)
}
```

`if let` is still useful when only one branch matters. `let-else` is good for
early returns, but its `else` block cannot bind the `Err` payload. A closure
such as `unwrap_or_else` can bind the error, but `return`, `break`, and
`continue` inside it do not control the caller. A macro keeps those choices
at the call site.

## Forms

Use `okerrr!` for `Result` and `okerrr_some!` for `Option` in new code.
`okerr!` and `okerr_some!` are supported convenience spellings.

```rust
use okerrr::{okerrr, okerrr_some};

fn double(input: Result<i32, &'static str>) -> Result<i32, String> {
    let value = okerrr!(input, error => return Err(format!("bad input: {}", error)));
    Ok(value * 2)
}

fn value_or_zero(input: Result<i32, &'static str>) -> i32 {
    okerrr!(input, 0) // also: okerrr!(input, else 0)
}

fn maybe_double(input: Option<i32>) -> Option<i32> {
    let value = okerrr_some!(input);
    Some(value * 2)
}

fn option_or_zero(input: Option<i32>) -> i32 {
    okerrr_some!(input, 0) // also: okerrr_some!(input, else 0)
}
```

`okerrr!(result)` returns `Err(From::from(error))` from the caller on failure.
An error handler can also `break` or `continue` an enclosing loop. The input
expression runs once; fallback expressions run only for `Err` or `None`.

The default build has no dependencies and supports `#![no_std]`.

## Caller-side tracing

`okerrr!` does not log an `Err` or require its type to implement a formatting
trait. If a caller decides an error deserves an event, put `tracing::error!`
in the bound handler:

```toml
[dependencies]
okerrr = "0.1"
tracing = "0.1"
```

```rust
use okerrr::okerrr;

fn read(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
    let value = okerrr!(input, error => {
        tracing::error!(error = ?error, "read failed");
        return Err(error);
    });
    Ok(value)
}
```

The `Debug` requirement above comes from the caller's `?error` field. A
caller can choose different fields or a different level. The application owns
its subscriber and any tracing-to-OpenTelemetry pipeline; this crate has no
exporter or tracing feature.

## Contributor checks

Activate the tracked `pre-commit` hook in this checkout:

```sh
git config core.hooksPath .githooks
```

It checks formatting and strict Clippy for the crate and its downstream
`no_std` fixture. It stops a commit if either check fails. Run
`cargo fmt --all` and
`cargo fmt --manifest-path tests/fixtures/downstream/Cargo.toml`, then
restage reviewed changes before committing.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
