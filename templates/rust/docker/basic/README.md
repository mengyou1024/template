# {{project-name}}

A minimal Rust app template with Docker multi-stage build support.

- Rust edition: `2024`
- Package version: `0.1.0`
- Builder image: `rust:slim`

## Local run

```bash
cargo run
```

## Build image

```bash
docker build -t {{project-name}} .
```

## Run container

```bash
docker run --rm {{project-name}}
```
