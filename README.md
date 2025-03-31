# Zero to production in rust

[![Rust](https://github.com/pedrorpmoleiro/zero2prod/actions/workflows/general.yaml/badge.svg?event=push)](https://github.com/pedrorpmoleiro/zero2prod/actions/workflows/general.yaml)

This repository will host the implementation following the book [Zero to production in Rust](https://www.zero2prod.com/index.html).

## Email newsletter management cloud native application

## Web Framework

For this project the `actix-web` was chosen as the go to web framework due to it being aimed at production usage, extensive use and large community.

## Turning the project into a library for testing

After doing the transition of the code into the `lib.rs` file the bellow configurations are no longer required to be added to the `cargo.toml` file for the project and tests to work.

```toml
[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "zero2prod"
```

## Tools used during development

- [`bacon`](https://dystroy.org/bacon/) in replacement of `cargo-watch` which is no longer maintained. Install with `cargo install bacon`
- [`cargo-tarpaulin`](https://github.com/xd009642/tarpaulin) to compute code coverage. Install with `cargo install cargo-tarpaulin`
- Official `clippy` for linting. Install with `rustup component add clippy`
- Official `rustfmt` for linting. Install with `rustup component add rustfmt`
- `cargo-audit` for security vulnerability checking. Install with `cargo install cargo-audit`
- `sqlx-cli` for database migrations. Requires an environment variable with a value like `postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}`. 
