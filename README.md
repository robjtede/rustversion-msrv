> Conditional compilation according manifest MSRV.

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/rustversion-msrv?label=latest)](https://crates.io/crates/rustversion-msrv)
[![Documentation](https://docs.rs/rustversion-msrv/badge.svg?version=0.99.16)](https://docs.rs/rustversion-msrv/0.99.16)
![Version](https://img.shields.io/badge/rustc-1.63+-ab6000.svg)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/rustversion-msrv.svg)
<br />
[![dependency status](https://deps.rs/crate/rustversion-msrv/0.99.16/status.svg)](https://deps.rs/crate/rustversion-msrv/0.99.16)
![Download](https://img.shields.io/crates/d/rustversion-msrv.svg)

<!-- prettier-ignore-end -->

<!-- cargo-rdme start -->

Conditional compilation according manifest MSRV.

# Selectors

- `#[rustversion_msrv::msrv]`

  True on the **call-site** crate's `rust-version` field, i.e., its minimum supported Rust
  version (MSRV).

# Use Cases

The motivating use case for the `msrv` macro in this crate is to ensure a stable compiler error
output when running negative trybuild tests. Guarding your test function like this means you
only need to update the .stderr files when you bump your MSRV, not (potentially) every stable
release (or worse). Of course, try make sure that your CI is actually running an MSRV job in its
set.

```rust
#[rustversion_msrv::msrv]
#[test]
fn trybuild() {
   // ...
}
```

<!-- cargo-rdme end -->
