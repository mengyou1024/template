# {{project-name}}

A Rust CLI application template built with [`clap`](https://github.com/clap-rs/clap).

- Rust edition: `2024`
- CLI framework: `clap` (derive)
- Error handling: `anyhow`
- Logging: `tracing` + `tracing-subscriber`

## Quick Start

```bash
cargo run -- greet --help
cargo run -- greet World
cargo run -- greet Alice -c 3 -s pirate
cargo run -- --verbose greet Bob
```

## Available commands

- `greet` — Greet someone with customizable style and count
- `config` — Show application configuration

## Global flags

- `-v`, `--verbose` — Increase verbosity (can be used multiple times: -v, -vv, -vvv)
- `-q`, `--quiet` — Suppress all output except errors
- `-h`, `--help` — Print help information
- `-V`, `--version` — Print version information

## Build

```bash
cargo build --release
```
