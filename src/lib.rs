//! Add a cute dependency declaration snippet in your crate documentation.
//!
//! # Adding to `Cargo.toml`
#![doc = dep_doc!()]
//!
//! # Goal
//!
//! This crate aims to automate the dependency declaration snippet generation.
//! It is really easy to forget to update it before running `cargo publish` or
//! `cargo release`. This crate completely removes this source of error by
//! providing the [`dep_doc`] macro, which always expands to the crate declared
//! in the crate's `Cargo.toml` file.
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
//! That's fine! [`dev_dep_doc`] generates the appropriate documentation.

#[doc(hidden)]
pub use core;

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
