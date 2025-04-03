#FROM lukemathwalker/cargo-chef:latest-rust-1.85.1 AS chef
#FROM rust:1.85.1 AS builder

FROM lukemathwalker/cargo-chef:latest-rust-1.85.1-alpine AS chef
#FROM rust:1.85.1-alpine AS builder

WORKDIR /app
#RUN apt-get update -y && apt-get install lld clang -y
RUN apk add --no-cache lld clang

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

#FROM debian:bookworm-slim AS runtime
FROM alpine:3.21 AS runtime

WORKDIR /app
#RUN apt-get update -y && \
#    apt-get install -y --no-install-recommends openssl ca-certificates && \
#    apt-get autoremove -y && apt-get clean -y && \
#    rm -rf /var/lib/apt/lists/*
RUN apk add --no-cache openssl ca-certificates
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration

ENV APP_ENVIRONMENT production
EXPOSE 8000

ENTRYPOINT ["./zero2prod"]
