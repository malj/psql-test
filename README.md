# PSQL Testing Ground

This is a simple [Rust](https://www.rust-lang.org/) project which uses built in [Cargo](https://doc.rust-lang.org/cargo/) test runner and [SQLx](https://crates.io/crates/sqlx) library to setup and run [PostgreSQL](https://www.postgresql.org/) schema tests.

## Setup

1. Install [PostgreSQL](https://www.postgresql.org/download)

2. Install [Rust](https://www.rust-lang.org/learn/get-started)

3. Install SQLx CLI tool:

   ```sh
   cargo install sqlx-cli
   ```

4. Clone the project

   ```sh
   git clone git@github.com:malj/psql-test.git
   ```

5. Switch to the project root

   ```sh
   cd psql-test
   ```

6. Copy the example environment file in the project root:

   ```sh
   cp .env.example .env
   ```

7. Edit the environment variables in the copied `.env` file:

   - `DATABASE_URL`: Full connection string to a test database. The user must have full permissions to edit the database.

## Usage

### Migrating the schema

Apply the latest migrations:

```sh
sqlx migrate run
```

Revert a single migration, or optionally to a specific version (0 = all):

```sh
sqlx migrate revert [--target-version <VERSION>]
```

Create a new migration:

- `r` flag creates an up/down pair to enable reverting
- `s` flag uses sequential versioning instead of timestamps

```sh
sqlx migrate add -rs <NAME>
```

### Testing

After applying the required migrations, run the tests using Cargo test runner:

```sh
cargo test [<NAME_PATTERN>]
```

This will download all the dependencies and run all tests. If a name pattern is specified, only test functions with matching names will be executed.
