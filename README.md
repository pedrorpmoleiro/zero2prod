# Zero to production in rust

This repository will host the implementation following the book [Zero to production in Rust](https://www.zero2prod.com/index.html).

## Tools used during development

- [`bacon`](https://dystroy.org/bacon/) in replacement of `cargo-watch` which is no longer maintained. Install with `cargo install bacon`
- [`cargo-tarpaulin`](https://github.com/xd009642/tarpaulin) to compute code coverage. Install with `cargo install cargo-tarpaulin`
- Official `clippy` for linting. Install with `rustup component add clippy`
- Official `rustfmt` for linting. Install with `rustup component add rustfmt`
- `cargo-audit` for security vulnerability checking. Install with `cargo install cargo-audit`
