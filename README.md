# Zero to production in rust

[![Rust](https://github.com/pedrorpmoleiro/zero2prod/actions/workflows/general.yaml/badge.svg?event=push)](https://github.com/pedrorpmoleiro/zero2prod/actions/workflows/general.yaml)
[![Security audit](https://github.com/pedrorpmoleiro/zero2prod/actions/workflows/audit.yaml/badge.svg)](https://github.com/pedrorpmoleiro/zero2prod/actions/workflows/audit.yaml)

This repository will host the implementation following the
book [Zero to production in Rust](https://www.zero2prod.com/index.html).

## Email newsletter management cloud native application

## Web Framework

For this project the `actix-web` was chosen as the go-to web framework due to it being aimed at production usage,
extensive use and large community.

## Tools used during development

- [`bacon`](https://dystroy.org/bacon/) in replacement of `cargo-watch` which is no longer maintained. Install with
  `cargo install bacon`
- `llvm-cov` to compute code coverage.
- Official `clippy` for linting. Install with `rustup component add clippy`
- Official `rustfmt` for linting. Install with `rustup component add rustfmt`
- `cargo-audit` for security vulnerability checking. Install with `cargo install cargo-audit`
- `sqlx-cli` for database migrations. Requires an environment variable with a value like
  `postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}`.

## Container images

[Podman](https://podman.io/) is being used as the container runtime for the development of the project with the goal to
also learn Podman native development.

Because of this `Containerfile` is being used instead of `Dockerfile`.
The definition between these two file types are the same and Podman supports both, but `Containerfile` is the official
Podman container definition file so it is being used.
The same goes for `.containerignore` and the Docker equivalent `.dockerignore`.

## Email sending API

The book recommends the use of [Postmark](https://postmarkapp.com/), since now it only offers a free trial I will be
using [Mailersend](https://www.mailersend.com/) which offers a free tier with up to 3000 free emails a month.

## Valkey instead of Redis

[Valkey](https://valkey.io/) is an open source fork of [Redis](https://redis.io/) before Redis changed their license
from open source to
source available. This alternative can be used as a drop in replacement so it was chosen to keep with the open source
nature of this project.

## Admin credentials

Username: `admin`
Password: `everythinghastostartsomewhere`
