# okerrr!

A `no_std` macro for binding a `Result::Err` payload in a diverging `else`
branch.

[![Crates.io](https://img.shields.io/crates/v/okerrr.svg)](https://crates.io/crates/okerrr)
[![Documentation](https://docs.rs/okerrr/badge.svg)](https://docs.rs/okerrr)
[![Vulnerability scans](https://img.shields.io/github/actions/workflow/status/awill1988/okerrr/ci.yml?branch=main&event=push&label=vulnerability%20scans)](https://github.com/awill1988/okerrr/actions/workflows/ci.yml)
[![Code coverage](https://codecov.io/gh/awill1988/okerrr/graph/badge.svg?branch=main)](https://codecov.io/gh/awill1988/okerrr)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Bound `else`

Rust's [`let-else`](https://rust-lang.github.io/rfcs/3137-let-else.html)
keeps the successful value in the surrounding scope and requires the failure
branch to diverge. With a `Result`, however, its catch-all `else` pattern
cannot also bind the moved `Err` payload.

A `match` can bind that error and return from the caller:

```rust
fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
    let value = match input {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    Ok(value * 2)
}
```

`okerrr!` expresses the same control flow as a bound `else`:

```rust
use okerrr::okerrr;

fn process(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
    let value = okerrr!(input, else error => return Err(error));
    Ok(value * 2)
}
```

The macro has one form:

```rust
okerrr!(result_expression, else error_binding => diverging_handler)
```

The input expression runs once. An `Ok` produces its payload. An `Err` moves
its payload into the named binding, and the handler must `return`, `break`,
`continue`, panic, loop forever, or call another expression that never
returns. A value-producing fallback is rejected at compile time.

The handler expands directly in the caller, so its control flow applies to the
surrounding function or loop. A closure such as `unwrap_or_else` can bind the
error, but `return`, `break`, and `continue` inside a closure cannot control
the caller.

## Async expressions

Async needs no separate feature. Await the input expression where it is
produced:

```rust
use okerrr::okerrr;

async fn load() -> Result<i32, &'static str> {
    let value = okerrr!(fetch_value().await, else error => return Err(error));
    Ok(value)
}

async fn fetch_value() -> Result<i32, &'static str> {
    Ok(42)
}
```

The macro does not await implicitly.

## Caller-side tracing

`okerrr!` does not log or require the error type to implement `Debug` or
`Display`. A caller can instrument the branch explicitly:

```rust
use okerrr::okerrr;

fn read(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
    let value = okerrr!(input, else error => {
        tracing::error!(error = ?error, "read failed");
        return Err(error);
    });
    Ok(value)
}
```

The `Debug` requirement in this example comes from the caller's `?error`
field. The application owns its subscriber and any tracing-to-OpenTelemetry
pipeline.

The crate has no dependencies, enables no default features, and supports
`#![no_std]` on Rust `1.56` and later.

## The name

The name describes the domain first: **`Ok` + `Err` + `R(esult)` =
`okerrr!`**. Each part contributes meaning to the whole.

Read aloud, the extended `r` also gives a light phonetic nod to *okurrr*, a
trilled “okay” associated with drag culture and widely popularized by Cardi B.
The [cultural reference](https://www.dictionary.com/culture/pop-culture/okurrr)
sets the tone; the macro's contract comes from Rust.

## Contributor checks

Install the conventional commit linter and activate the tracked hooks:

```sh
npm ci
git config core.hooksPath .githooks
```

`pre-commit` checks formatting and strict Clippy for the crate and its
downstream `no_std` fixture. `commit-msg` checks conventional commit format.
Both stop invalid commits. Run `cargo fmt --all` and
`cargo fmt --manifest-path tests/fixtures/downstream/Cargo.toml`, then restage
reviewed changes before committing.

Squash merges use the pull request title as the final commit message. Mark
breaking changes with `!` in that title, such as
`feat!: define bound else contract`, so release notes retain the signal.

## Releases

Ordinary pull requests keep the version in `Cargo.toml` unchanged. Their
merges run CI and do not publish.

To stage a release, run the `Prepare Release` workflow with an exact Cargo
version such as `0.1.0-rc.1`. The workflow validates the version against
`main` and crates.io, creates `release/v0.1.0-rc.1`, and opens a dedicated
release pull request. CI validates the package and attaches a release-notes
preview to that pull request.

Merging the release pull request publishes its exact manifest version to
crates.io and creates a matching GitHub release. Versions such as
`0.1.0-rc.1` become prerelease GitHub releases and are not marked latest.
Promote a candidate by running `Prepare Release` again with the next candidate
or the stable version, such as `0.1.0`. Prerelease tags do not truncate the
stable release notes, so the stable notes retain the complete change set.

The release job creates the tag and a draft GitHub release before uploading
to crates.io. A rerun can finish a partial release only when that tag still
points to the same `main` commit. Any other duplicate version fails closed.
Notes live in GitHub releases; there is no tracked changelog file.

The published `0.0.0` package is the bootstrap baseline. The first version
increase from it triggers publishing. Later increases require the previous
version to be published. Run `CI` manually from GitHub Actions to check
packaging and production secret access without publishing.

Generated release commits use `chore(release): prepare <version>`; release
notes omit those commits.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
