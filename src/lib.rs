//! Conditional compilation according manifest MSRV.
//!
//! # Selectors
//!
//! - `#[rustversion_msrv::msrv]`
//!   
//!   True on the **call-site** crate's `rust-version` field, i.e., its minimum supported Rust
//!   version (MSRV).
//!
//! # Use Cases
//!
//! The motivating use case for the `msrv` macro in this crate is to ensure a stable compiler error
//! output when running negative trybuild tests. Guarding your test function like this means you
//! only need to update the .stderr files when you bump your MSRV, not (potentially) every stable
//! release (or worse). Of course, try make sure that your CI is actually running an MSRV job in its
//! set.
//!
//! ```
//! #[rustversion_msrv::msrv]
//! #[test]
//! fn trybuild() {
//!    // ...
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;

mod error;
mod expand;
mod expr;
mod iter;
mod release;
mod token;
mod version;

use self::version::Version;

const RUST_VERSION: Version = include!(concat!(env!("OUT_DIR"), "/version.expr"));

#[proc_macro_attribute]
pub fn msrv(args: TokenStream, input: TokenStream) -> TokenStream {
    expand::cfg("msrv", args, input)
}
