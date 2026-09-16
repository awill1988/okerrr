# okerrr 💅

Rust macros to collapse tedious multi-line `match` patterns for `Ok`/`Err` and `Option` handling into clean 1-liners while binding error payloads and maintaining full control flow.

[![Crates.io](https://img.shields.io/crates/v/okerrr.svg)](https://crates.io/crates/okerrr)
[![Documentation](https://docs.rs/okerrr/badge.svg)](https://docs.rs/okerrr)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

---

## Why `okerrr`?

In standard Rust:
- **`let Ok(val) = expr else { return ... };`** (Rust 1.65+ `let-else`) is great for early return, BUT **it throws away the `Err(e)` payload**. You cannot bind `e` inside the `else` block!
- **`.unwrap_or_else(|err| ...)`** lets you inspect `err`, BUT closures **cannot `return`, `break`, or `continue`** out of the enclosing function or loop.
- Standard **`match`** statements require 3–5 lines of boilerplate every single time you want to log, inspect, or transform an error while controlling function flow.

`okerrr!` solves this annoyance in 1 line by harmonizing `Ok` and `Err`:

```rust
use okerrr::okerrr;

fn process_data(res: Result<Data, MyError>) -> Result<Output, MyError> {
    // Bind error payload 'err' AND retain early return control flow!
    let data = okerrr!(res, err => {
        eprintln!("Failed to fetch data: {err}");
        return Err(err);
    });

    Ok(data.into_output())
}
```

---

## Features

- ⚡ **Zero Overhead & `#![no_std]`**: Pure declarative macro with zero dependencies by default.
- 💅 **Ergonomic Aliases**: Re-exported as `okerr!`, `okurrr!`, and `okurr!` for maximum convenience.
- 🔭 **Cloud-Native / OTel Auto-Instrumentation**: Feature-gated `tracing` / `otel` support automatically emits error events into your tracing pipeline.
- 🔄 **Loop Control Flow**: Native support for `break` and `continue` inside error handling blocks.

---

## Usage Patterns

### 1. Bind Error Payload with Control Flow
```rust
let val = okerrr!(res, err => return Err(err.into()));
let item = okerrr!(res, else err => { log::error!("{err}"); break; });
```

### 2. Fallback Expression
```rust
let count = okerrr!(res, else 0);
let count = okerrr!(res, 0); // 2-arg shorthand
```

### 3. Early Return Shorthand (`Result`)
```rust
let val = okerrr!(res); // Returns Err(err.into()) on failure
```

### 4. `Option<T>` Pattern (`okerrr_some!`)
```rust
let val = okerrr_some!(opt); // Returns None on failure
let val = okerrr_some!(opt, else "default_val");
```

---

## OpenTelemetry & Tracing Integration

Add the `otel` or `tracing` feature flag to `Cargo.toml`:

```toml
[dependencies]
okerrr = { version = "0.1", features = ["otel"] }
```

When enabled, any caught `Err(err)` will automatically record a `tracing::error!(target: "okerrr", error = %err, ...)` event before executing your handler block!

---

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).


