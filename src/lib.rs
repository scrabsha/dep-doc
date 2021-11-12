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
//! ## Using a path instead of a version
//!
//! It is not allowed to specify both a `version` and a `path` in a dependency
//! declaration. As such, passing a `path = "..."` argument to `dep_doc`, will
//! remove the `version` section:
//!
//! ```rust
//! //! Some doc...
//! #![doc = dep_doc::dep_doc!(path = "https://github.com/scrabsha/dep-doc")]
//! //! Some other doc
//! ```
//!
//! If invoked in `dep_doc`, this will generate the following documentation:
//!
//! > Some doc...
//! > ```TOML
//! > [dependencies]
#![doc = concat!("> ", package_import!(path = "https://github.com/scrabsha/dep-doc"))]
//! > ```
//!
//! ## Other customizations
//!
//! Some crates may document specific features, git repository, branches,
//! commit hash, and so on. This can be addressed by passing code in the
//! [`dep_doc`] invocation:
//!
//! ```rust
//! //! Some doc...
//! #![doc = dep_doc::dep_doc!(features = ["parse"])]
//! //! Some other doc
//! ```
//!
//! If invoked in `dep_doc`, this will generate the following documentation:
//!
//! > Some doc...
//! > ```TOML
#![doc = concat!("> ", package_import!(features = ["parse"]))]
//! > ```
//! > Some other doc.
//!
//! # My library is better suited as a development dependency
//!
//! That's fine! [`dev_dep_doc`] generates the appropriate documentation. It
//! replaces the `[dependencies]` section by a `[dev-dependencies]` one.

#[doc(hidden)]
pub use core;

/// Generates a manifest
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
    (path = $path:literal) => {
        concat!(
            crate::core::env!("CARGO_PKG_NAME"),
            " = { path = \"",
            $path,
            "\" }"
        )
    };
    () => {
        concat!(
            $crate::core::env!("CARGO_PKG_NAME"),
            " = \"",
            $crate::core::env!("CARGO_PKG_VERSION"),
            "\"",
        )
    };

    (path = $path:literal $( $tt:tt )+ ) => {
        concat!(
            $crate::core::env!("CARGO_PKG_NAME"),
            " = { path = \"",
            $path,
            "\", ",
            stringify!( $( $tt )+ ),
            " }",
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
