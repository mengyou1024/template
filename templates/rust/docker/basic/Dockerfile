# syntax=docker/dockerfile:1

FROM rust:slim AS builder
WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/{{project-name}} /usr/local/bin/{{project-name}}

ENTRYPOINT ["/usr/local/bin/{{project-name}}"]
