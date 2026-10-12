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
COPY {{ project-name }}-app/Cargo.toml {{ project-name }}-app/Cargo.toml

# Create dummy source files to pre-compile dependencies
RUN mkdir -p entity/src migration/src {{ project-name }}-app/src && \
    echo "pub fn dummy() {}" > entity/src/lib.rs && \
    echo "fn main() {}" > migration/src/main.rs && \
    echo "pub fn dummy() {}" > migration/src/lib.rs && \
    echo "fn main() {}" > {{ project-name }}-app/src/main.rs

# Build dependencies only
RUN cargo build --release --workspace

# Now copy actual source code
COPY entity/src entity/src
COPY migration/src migration/src
COPY {{ project-name }}-app/src {{ project-name }}-app/src

# Force rebuild of our crates
RUN touch entity/src/lib.rs migration/src/lib.rs migration/src/main.rs {{ project-name }}-app/src/main.rs && \
    cargo build --release --workspace

# =========================================
# Migration stage (static musl binary)
# =========================================
FROM alpine:3 AS migration

# tzdata 提供时区文件,使 TZ 环境变量生效(日志时间/本地时间解析依赖)
RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/migration /usr/local/bin/migration

ENTRYPOINT ["migration"]

# =========================================
# Runtime stage (static musl binary)
# =========================================
FROM alpine:3 AS runtime

# tzdata 提供时区文件,使 TZ 环境变量生效(日志时间/本地时间解析依赖)
RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/{{ project-name }}-app /usr/local/bin/{{ project-name }}-app

EXPOSE 3000

ENTRYPOINT ["{{ project-name }}-app"]
