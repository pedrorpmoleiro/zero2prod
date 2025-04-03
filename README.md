# Zero to production in rust

[![Rust](https://github.com/pedrorpmoleiro/zero2prod/actions/workflows/general.yaml/badge.svg?event=push)](https://github.com/pedrorpmoleiro/zero2prod/actions/workflows/general.yaml)
![Code Coverage](https://img.shields.io/badge/Code%20Coverage-76%25-success?style=flat)

This repository will host the implementation following the book [Zero to production in Rust](https://www.zero2prod.com/index.html).

## Email newsletter management cloud native application

## Web Framework

For this project the `actix-web` was chosen as the go-to web framework due to it being aimed at production usage, extensive use and large community.

## Tools used during development

- [`bacon`](https://dystroy.org/bacon/) in replacement of `cargo-watch` which is no longer maintained. Install with `cargo install bacon`
- `llvm-cov` to compute code coverage.
- Official `clippy` for linting. Install with `rustup component add clippy`
- Official `rustfmt` for linting. Install with `rustup component add rustfmt`
- `cargo-audit` for security vulnerability checking. Install with `cargo install cargo-audit`
- `sqlx-cli` for database migrations. Requires an environment variable with a value like `postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}`. 
