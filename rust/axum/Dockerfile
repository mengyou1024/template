# syntax=docker/dockerfile:1.8

# =========================================
# Builder stage
# =========================================
FROM rust:1-trixie AS builder

WORKDIR /app

# Install musl toolchain for static linking
RUN apt-get update && apt-get install -y --no-install-recommends \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

# Set default target to musl
ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl

# Copy workspace manifests and build dependencies first (caching layer)
COPY Cargo.toml ./
COPY entity/Cargo.toml entity/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml
COPY app/Cargo.toml app/Cargo.toml

# Create dummy source files to pre-compile dependencies
RUN mkdir -p entity/src migration/src app/src && \
    echo "pub fn dummy() {}" > entity/src/lib.rs && \
    echo "fn main() {}" > migration/src/main.rs && \
    echo "pub fn dummy() {}" > migration/src/lib.rs && \
    echo "fn main() {}" > app/src/main.rs

# Build dependencies only
RUN cargo build --release --workspace

# Now copy actual source code
COPY entity/src entity/src
COPY migration/src migration/src
COPY app/src app/src

# Force rebuild of our crates
RUN touch entity/src/lib.rs migration/src/lib.rs migration/src/main.rs app/src/main.rs && \
    cargo build --release --workspace

# =========================================
# Migration stage (static musl binary)
# =========================================
FROM alpine:3 AS migration

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/migration /usr/local/bin/migration

ENTRYPOINT ["migration"]

# =========================================
# Runtime stage (static musl binary)
# =========================================
FROM alpine:3 AS runtime

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/app

EXPOSE 3000

ENTRYPOINT ["app"]
