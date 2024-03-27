# PSQL Testing Ground

This is a simple Rust project which uses built in [Cargo](https://doc.rust-lang.org/cargo/) test runner and [SQLx](https://crates.io/crates/sqlx) library to setup and run [PostgreSQL](https://www.postgresql.org/) schema tests.

## Install

Install SQLx CLI tool:

```sh
cargo install sqlx-cli
```

Copy the example environment file:

```
cp .env.example .env
```

Edit the environment variables in the copied file:

- `DATABASE_URL`: Full connection string to a test database. The user must have full permissions to edit the database.

## Migrations

Create a new migration:

- `r` flag creates an up/down pair to enable reverting
- `s` flag uses sequential versioning instead of timestamps

```sh
sqlx migrate add -rs <NAME>
```

Apply the latest migrations:

```sh
sqlx migrate run
```

Revert a single migration, or optionally to a specific version (0 = all):

```sh
sqlx migrate revert [--target-version <VERSION>]
```
