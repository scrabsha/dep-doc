//! Add a cute dependency declaration snippet in your crate documentation.
//!
//! # Adding to `Cargo.toml`
//!
#![doc = dep_doc!()]
//!
//! # Goal
//!
//! When writing Rust libraries, it is quite common to add a code snippet
//! which shows to the end-user how to add the crate to the `Cargo.toml`. The
//! problem is that there's no way to ensure that the crate name and version
//! are correct before releasing a new version.
//!
//! This crate aims to automate the TOML snippet generation by providing
//! macro-assisted solution which expand in the correct crate name and version.
//! It does so by reading environment variables that are set by cargo itself.
//!
//! # Set up
//!
//! To add the TOML snippet, insert the following line between two documentation
//! lines:
//!
//! ```rust
//! //! Some doc...
//! #![doc = dep_doc::dep_doc!()]
//! //! Some other doc.
//! ```
//!
//! If invoked in `dep_doc`, this will generates the following documentation:
//!
//! > Some doc...
//! > ```TOML
//! > [dependencies]
#![doc = concat!("> ", package_import!())]
//! > ```
//! > Some other doc.
//!
//! # Customization
//!
//! Some crates may document specific features, git repository, branches,
//! commit hash, and so on. This can be addressed by passing code in the
//! [`dep_doc`] invocation:
//!
//! ```rust
//! //! Some doc...
//! #![doc = dep_doc::dep_doc!(git = "https://github.com/scrabsha/dep-doc")]
//! //! Some other doc
//! ```
//!
//! If invoked in `dep_doc`, this will generate the following documentation:
//!
//! > Some doc...
//! > ```TOML
#![doc = concat!("> ", package_import!(git = "https://github.com/scrabsha/dep-doc"))]
//! > ```
//! > Some other doc.
//!
//! # My library is better suited as a development dependency
//!
//! That's fine! [`dev_dep_doc`] generates the appropriate documentation. It
//! replaces the `[dependencies]` section by a `[dev-dependencies]` one.

#[doc(hidden)]
pub use core;

/// Generates a `Cargo.toml` code snippet showing how to add the current crate
/// as a dependency.
///
/// This crate expands to the correct docstring. As such, the `#![doc = ...]`
/// macro must be used.
///
/// See the [crate-level documentation][crate] for more.
///
/// # Example
///
/// The simplest invocation is:
///
/// ```rust
/// #![doc = dep_doc::dep_doc!()]
/// ```
///
/// Specific feature, git repository, path can be passed in the macro invocation:
///
/// ```rust
/// #![doc = dep_doc::dep_doc!(git = "https://github.com/scrabsha/dep-doc")]
/// ```
#[macro_export]
macro_rules! dep_doc {
    ( $( $tt:tt )* ) => {
        concat!(
            "```TOML\n[dependencies]\n",
            $crate::package_import!(),
            "\n```",
        )
    };
}

/// Generates a `Cargo.toml` code snippet showing how to add the current crate
/// as a dev-dependency.
///
/// # Example
///
/// The simplest invocation is:
///
/// ```rust
/// #![doc = dep_doc::dev_dep_doc!()]
/// ```
///
/// Specific feature, git repository, path can be passed in the macro invocation:
///
/// ```rust
/// #![doc = dep_doc::dev_dep_doc!(git = "https://github.com/scrabsha/dep-doc")]
/// ```

#[macro_export]
macro_rules! dev_dep_doc {
    ( $( $tt:tt )* ) => {
        concat!(
            "```TOML\n[dev-dependencies]\n",
            $crate::package_import!(),
            "\n```",
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! package_import {
    () => {
        concat!(
            $crate::core::env!("CARGO_PKG_NAME"),
            " = \"",
            $crate::core::env!("CARGO_PKG_VERSION"),
            "\"",
        )
    };

    ( $( $tt:tt)+ ) => {
        concat!(
            $crate::core::env!("CARGO_PKG_NAME"),
            " = { version = \"",
            $crate::core::env!("CARGO_PKG_VERSION"),
            "\", ",
            stringify!( $( $tt )+ ),
            " }",
        )
    }
}