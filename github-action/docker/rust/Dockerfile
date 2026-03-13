# syntax=docker/dockerfile:1

FROM rust:1-trixie AS builder
WORKDIR /workspace

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /workspace/target/x86_64-unknown-linux-musl/release/{{project-name}} /{{project-name}}

ENTRYPOINT ["/{{project-name}}"]
