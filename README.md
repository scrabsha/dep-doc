# `dep_doc`

[![Build Status][actions-badge]][actions-url]
[![Latest Version][version-badge]][version-url]
[![Rust Documentation][docs-badge]][docs-url]

[actions-badge]: https://github.com/scrabsha/dep-doc/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/scrabsha/dep-doc/actions/workflows/ci.yml?query=branch%3Amain
[version-badge]: https://img.shields.io/crates/v/dep_doc.svg
[version-url]: https://crates.io/crates/dep_doc
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg
[docs-url]: https://docs.rs/dep_doc


Add a cute dependency declaration snippet in your crate documentation.

## Adding to `Cargo.toml`

```TOML
[dependencies]
dep_doc = "0.1.1"
```

## Goal

When writing Rust libraries, it is quite common to add a code snippet
which shows to the end-user how to add the crate to the `Cargo.toml`. The
problem is that there's no way to ensure that the crate name and version
are correct before releasing a new version.

This crate aims to automate the TOML snippet generation by providing
macro-assisted solution which expand in the correct crate name and version.
It does so by reading environment variables that are set by cargo itself.

## Usage

To add the TOML snippet, insert the following line between two documentation
lines:

```rust
//! Some doc...
#![doc = dep_doc::dep_doc!()]
//! Some other doc
```

If invoked in `dep_doc`, this will generates the following documentation:

> Some doc...
> ```TOML
> [dependencies]
> dep_doc = "0.1.1"
> ```
> Some other doc

## Customization

Some crates may document specific features, git repository, branches,
commit hash, and so on. This can be addressed by passing code in the
`dep_doc` invocation:

```rust
//! Some doc...
#![doc = dep_doc::dep_doc!(git = "https://github.com/scrabsha/dep-doc")]
//! Some other doc
```

If invoked in `dep_doc`, this will generate the following documentation:

> Some doc...
> ```TOML
> [dependencies]
> dep_doc = { version = "0.1.1", git = "https://github.com/scrabsha/dep-doc" }
> ```
> Some other doc

## My library is better suited as a development dependency

That's fine! `dev_dep_doc` generates the appropriate documentation. It
replaces the `[dependencies]` section by a `[dev-dependencies]` one.

```rust
//! Some doc...
#![doc = dep_doc::dev_dep_doc!(features = ["proc_macro", "no_std"])]
//! Some other doc
```

If invoked in `dep_doc`, this will generate the following documentation:

> Some doc...
> ```TOML
> [dev-dependencies]
> dep_doc = { version = "0.1.1", features = ["prod_macro", "no_std"] }
> ```
> Some other doc


<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
