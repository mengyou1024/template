# syntax=docker/dockerfile:1

FROM rust:1-trixie AS builder
WORKDIR /workspace

COPY . .

RUN cargo build --release

FROM debian:trixie-slim AS runtime
WORKDIR /workspace

COPY --from=builder /workspace/target/release/{{project-name}} /usr/local/bin/{{project-name}}

ENTRYPOINT ["/usr/local/bin/{{project-name}}"]
