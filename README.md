# sqlc-gen-rust

A [sqlc](https://docs.sqlc.dev/) plugin that generates type-safe Rust code from SQL queries.

## Features

- Generates Rust structs from database tables
- Creates type-safe query functions with sqlx integration
- Supports PostgreSQL data types with proper Rust type mapping
- Handles nullable columns with `Option<T>`
- Generates enums from database enums
- Async/await support with sqlx

## Installation

Build the plugin from source:

```bash
cargo build --release
```

The binary will be available at `target/release/sqlc-gen-rust`.

## Configuration

Add the plugin to your `sqlc.yaml` configuration:

```yaml
version: '2'
plugins:
  - name: rust
    process:
      cmd: ./target/release/sqlc-gen-rust

sql:
  - schema: schema.sql
    queries: queries.sql
    engine: postgresql
    codegen:
      - plugin: rust
        out: src/db
        options:
          package: db
          emit_json_tags: true
          json_tags_case_style: snake_case
          output_models_file_name: models.rs
          output_db_file_name: queries.rs
```

## Plugin Options

- `emit_json_tags`: Include serde Serialize/Deserialize derives (default: true)
- `json_tags_case_style`: Case style for JSON field names (default: "snake_case")
- `output_models_file_name`: Name of the models file (default: "models.rs")
- `output_db_file_name`: Name of the queries file (default: "queries.rs")

## Generated Code Structure

The plugin generates three files:

1. **models.rs**: Contains struct definitions for database tables and enums
2. **queries.rs**: Contains the `Database` struct with async query methods
3. **lib.rs**: Module exports

## Example

Given a schema:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

And a query:

```sql
-- name: GetUser :one
SELECT id, name, email, created_at FROM users WHERE id = $1;
```

The plugin generates:

```rust
// models.rs
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "snake_case")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

// queries.rs
impl Database {
    pub async fn get_user(&self, param_1: i32) -> Result<User, SqlxError> {
        // Generated query implementation
    }
}
```

## Type Mappings

| SQL Type | Rust Type |
|----------|-----------|
| INTEGER, INT4 | i32 |
| BIGINT, INT8 | i64 |
| SMALLINT, INT2 | i16 |
| REAL, FLOAT4 | f32 |
| DOUBLE PRECISION, FLOAT8 | f64 |
| NUMERIC, DECIMAL | rust_decimal::Decimal |
| TEXT, VARCHAR | String |
| BOOLEAN | bool |
| BYTEA | Vec<u8> |
| UUID | uuid::Uuid |
| TIMESTAMP | chrono::NaiveDateTime |
| TIMESTAMPTZ | chrono::DateTime<chrono::Utc> |
| DATE | chrono::NaiveDate |
| JSON, JSONB | serde_json::Value |

Nullable columns are wrapped in `Option<T>`, and array types become `Vec<T>`.

## Usage in Your Application

```rust
use sqlx::postgres::PgPool;
use your_app::db::{Database, User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect("postgresql://...").await?;
    let db = Database::new(pool);
    
    let user = db.get_user(1).await?;
    println!("User: {:?}", user);
    
    Ok(())
}
```

## Development

To contribute to this plugin:

1. Clone the repository
2. Run `cargo test` to run tests
3. Build with `cargo build --release`

## License

MIT License