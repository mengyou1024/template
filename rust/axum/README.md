# {{project-name}}

An Axum-based web application template with Sea-ORM, validator, and Docker support.

## Features

- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) v0.7 with ergonomic routing
- **Database**: [Sea-ORM](https://www.sea-ql.org/SeaORM/) with PostgreSQL (migration + entity crates)
- **Validation**: [validator](https://github.com/Keats/validator) with derive macros
- **Workspace**: Multi-crate workspace (`app`, `entity`, `migration`)
- **API Docs**: [utoipa](https://github.com/juhaku/utoipa) + Swagger UI at `/swagger-ui`
- **Configuration**: `figment` + `dotenvy` (TOML file + env vars)
- **Logging**: `tracing` with JSON output
- **Password Hashing**: Argon2 via `password-hash`
- **Docker**: Multi-stage Dockerfile + docker-compose with PostgreSQL
- **Dev Container**: VS Code Dev Container with PostgreSQL sidecar
- **CI**: GitHub Actions workflow (test + clippy)

## Project Structure

```
.
├── app/              # Axum web application
│   └── src/
│       ├── main.rs
│       ├── config.rs
│       ├── error.rs
│       ├── state.rs
│       ├── middleware.rs
│       └── routes/
│           ├── mod.rs
│           ├── health.rs
│           └── users/
│               ├── mod.rs
│               └── schemas.rs
├── entity/           # Sea-ORM entity models
│   └── src/
│       ├── lib.rs
│       ├── prelude.rs
│       └── user.rs
├── migration/        # Database migrations
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       └── m20250101_000001_create_user_table.rs
├── .devcontainer/    # VS Code Dev Container
├── .github/
│   └── workflows/    # CI/CD pipelines
├── docker-compose.yml
├── Dockerfile
└── Cargo.toml        # Workspace root
```

## Quick Start

### Prerequisites

- Rust 1.85+
- PostgreSQL 15+ (or use Docker)
- Docker & Docker Compose (optional, for database)

### 1. Start the database

```bash
docker compose up -d db
```

### 2. Configure environment

```bash
cp .env.example .env
```

### 3. Run migrations

```bash
cargo run -p migration
```

### 4. Start the server

```bash
cargo run -p app
```

The API will be available at `http://localhost:3000`.
Swagger UI is at `http://localhost:3000/swagger-ui`.

## Docker Deployment

Build and run everything with docker-compose:

```bash
docker compose up -d --build
```

## Development

### Adding a new migration

```bash
# Add a new migration file
# migration/src/mYYYYMMDD_HHMMSS_description.rs

# Register it in migration/src/main.rs
```

### Regenerating entities

```bash
# Install sea-orm-cli
cargo install sea-orm-cli

# Generate entities from database
sea-orm-cli generate entity -o entity/src --database-url postgres://postgres:postgres@localhost:5432/{{project-name}}
```

### Running tests

```bash
cargo test --workspace
```

### Running clippy

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

## API Endpoints

| Method | Path            | Description       |
|--------|-----------------|-------------------|
| GET    | `/health`       | Health check      |
| GET    | `/api/users`    | List users        |
| POST   | `/api/users`    | Create user       |
| GET    | `/api/users/:id`| Get user by ID    |
| PUT    | `/api/users/:id`| Update user       |
| DELETE | `/api/users/:id`| Delete user       |

Full interactive documentation at `/swagger-ui`.
