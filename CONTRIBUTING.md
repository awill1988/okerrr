# Contributing to `okerrr`

Thank you for helping make `okerrr` clearer, safer, and more useful. Bug
reports, design questions, documentation corrections, tests, and code are all
valuable contributions.

This project uses an issue-first workflow. It gives contributors an early
design signal and prevents two people from solving the same problem in
different directions.

## Before writing a change

Search the [open issues] and [closed issues] for related work. If no accepted
issue covers the change, open one and describe:

- the problem or limitation;
- the behavior you propose;
- a small example when the public macro contract is involved; and
- any compatibility, `no_std`, or minimum Rust version implications you see.

Wait for the maintainer to acknowledge the issue before starting an
implementation. An issue is ready for community implementation only when the
maintainer adds the `status: accepted` label or explicitly comments that it is
ready. A request for more information, a general acknowledgment, or continued
design discussion is not approval to implement.

This gate applies to pull requests opened by anyone other than
[`@awill1988`]. The maintainer's own changes are exempt because their
authorship supplies the prioritization and design decision directly. The same
technical, test, and review standards apply to every pull request.

An early pull request is not treated as an implementation candidate. The
maintainer will point it to this guide and may close it without a technical
review. That response is about sequencing, not the merit of the person, idea,
or code. After the related issue is accepted, the contributor is welcome to
reopen the pull request or submit a focused replacement.

Small typo fixes still benefit from an issue, even if the issue and acceptance
happen in the same exchange. Issues labeled `good first issue` or
`help wanted` are already accepted unless a maintainer comment says otherwise.

## Maintainer commitments

The issue-first gate creates a matching obligation for the maintainer. The
maintainer will:

- acknowledge a new issue within seven calendar days;
- state whether it is accepted, needs information, needs design discussion,
  duplicates existing work, or is outside the project's direction;
- give a reason when declining or closing a proposal;
- provide an initial review of an eligible pull request within seven calendar
  days;
- communicate when a review is blocked or delayed instead of leaving the
  contributor without status; and
- keep feedback specific, respectful, and focused on the change.

Acknowledgment does not guarantee acceptance or a merge. It guarantees a
clear response and a visible next state. If seven days pass without a response,
one polite follow-up on the issue or pull request is welcome.

## Design constraints

Changes must preserve the crate's core contract unless an accepted issue
explicitly changes it:

- `no_std` support remains the default;
- the crate has no default features or mandatory runtime dependencies;
- Rust `1.56` remains the minimum supported version;
- macro inputs are evaluated once;
- error payloads are moved without imposing `Debug` or `Display` bounds; and
- caller-controlled `return`, `break`, `continue`, and other diverging
  handlers retain their surrounding control-flow behavior.

Keep proposals narrow. Public macro syntax, minimum supported Rust version,
dependencies, release behavior, and licensing require design discussion before
implementation.

## Development setup

Rust and Cargo provide the commit tooling. Activate the repository hooks:

```sh
git config core.hooksPath .githooks
```

Create a focused branch in your fork. Commit messages must follow Conventional
Commits, use lowercase text, and keep the subject within 72 characters. Do not
add generated attribution, co-author footers, or tool signatures.

## Validate the change

Run the checks that match CI before requesting review:

```sh
cargo test
cargo check --no-default-features
cargo check --all-features
cargo fmt --all -- --check
cargo fmt --manifest-path tests/fixtures/downstream/Cargo.toml -- --check
cargo fmt --manifest-path tools/commit_check/Cargo.toml -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy \
  --manifest-path tests/fixtures/downstream/Cargo.toml \
  --all-targets \
  --features with_tracing \
  -- -D warnings
cargo clippy \
  --manifest-path tools/commit_check/Cargo.toml \
  --all-targets \
  -- -D warnings
python3 -m unittest discover -s scripts -p 'test_*.py'
cargo test --manifest-path tools/commit_check/Cargo.toml
cargo publish --dry-run --allow-dirty
```

Add focused tests for behavior changes. Update public documentation when the
accepted change alters syntax, semantics, compatibility, or contributor-facing
workflows.

## Open the pull request

Link the accepted issue and explain the resulting behavior. State the affected
modules and any manual validation. Keep one concern per pull request and keep
the branch current with `main` without rewriting shared history.

Review is a conversation. Answer questions, explain tradeoffs, and make review
updates visible in new commits until the change is approved. A green CI run is
required but does not replace maintainer review.

## Licensing

By contributing, you agree that your contribution is licensed under the same
[`MIT OR Apache-2.0`](README.md#license) terms as the project and that you have
the right to submit it under those terms.

[open issues]: https://github.com/awill1988/okerrr/issues
[closed issues]: https://github.com/awill1988/okerrr/issues?q=is%3Aissue%20state%3Aclosed
[`@awill1988`]: https://github.com/awill1988
