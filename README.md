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

### Option 1: Install from Source

```bash
git clone https://github.com/your-username/sqlc-gen-rust
cd sqlc-gen-rust
cargo build --release
```

The binary will be available at `target/release/sqlc-gen-rust`.

### Option 2: Install via Cargo

```bash
cargo install --path .
```

This will install the binary to your Cargo bin directory (usually `~/.cargo/bin/sqlc-gen-rust`).

### Option 3: Download Pre-built Binary

Download the latest release from the [GitHub releases page](https://github.com/rezi-labs/sqlr/releases) and place it in your PATH.

#### Available Platforms

- **Linux x86_64**: `sqlc-gen-rust-x86_64-unknown-linux-gnu.tar.gz`
- **Linux x86_64 (musl)**: `sqlc-gen-rust-x86_64-unknown-linux-musl.tar.gz` 
- **Linux ARM64**: `sqlc-gen-rust-aarch64-unknown-linux-gnu.tar.gz`
- **macOS x86_64**: `sqlc-gen-rust-x86_64-apple-darwin.tar.gz`
- **macOS ARM64 (Apple Silicon)**: `sqlc-gen-rust-aarch64-apple-darwin.tar.gz`
- **Windows x86_64**: `sqlc-gen-rust-x86_64-pc-windows-msvc.exe.zip`

#### Installation Steps

1. Download the appropriate binary for your platform from the releases page
2. Extract the archive (for Unix systems):
   ```bash
   tar -xzf sqlc-gen-rust-*.tar.gz
   ```
   Or unzip (for Windows):
   ```cmd
   unzip sqlc-gen-rust-*.zip
   ```
3. Move the binary to a directory in your PATH:
   ```bash
   # Unix systems
   sudo mv sqlc-gen-rust /usr/local/bin/
   
   # Or for user-local installation
   mv sqlc-gen-rust ~/.local/bin/
   ```
   ```cmd
   # Windows - move to a directory in your PATH
   move sqlc-gen-rust.exe C:\Users\%USERNAME%\bin\
   ```
4. Verify installation:
   ```bash
   sqlc-gen-rust --version
   ```

## Quick Start Tutorial

### Step 1: Install sqlc

If you don't have sqlc installed, install it first:

```bash
# On macOS
brew install sqlc

# On Linux/Windows, download from https://github.com/sqlc-dev/sqlc/releases
```

### Step 2: Set up your project

Create a new directory for your project:

```bash
mkdir my-rust-app
cd my-rust-app
```

### Step 3: Create your database schema

Create a `schema.sql` file:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Step 4: Write your queries

Create a `queries.sql` file:

```sql
-- name: GetUser :one
SELECT id, name, email, created_at FROM users WHERE id = $1;

-- name: ListUsers :many
SELECT id, name, email, created_at FROM users ORDER BY id;

-- name: CreateUser :one
INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email, created_at;
```

### Step 5: Configure sqlc

Create a `sqlc.yaml` file:

```yaml
version: '2'
plugins:
  - name: rust
    process:
      cmd: sqlc-gen-rust  # or ./target/release/sqlc-gen-rust if built from source

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

### Step 6: Generate Rust code

```bash
sqlc generate
```

This will create:
- `src/db/models.rs` - Struct definitions
- `src/db/queries.rs` - Query functions  
- `src/db/lib.rs` - Module exports

### Step 7: Use in your Rust application

Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

Use the generated code:

```rust
use sqlx::postgres::PgPool;
use crate::db::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect("postgresql://user:password@localhost/dbname").await?;
    let db = Database::new(pool);
    
    // Create a user
    let user = db.create_user("John Doe".to_string(), Some("john@example.com".to_string())).await?;
    println!("Created user: {:?}", user);
    
    // Get the user
    let user = db.get_user(user.id).await?;
    println!("Retrieved user: {:?}", user);
    
    Ok(())
}
```

## Configuration

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

## Advanced Plugin Integration

### Using with Different Database Engines

While this plugin is optimized for PostgreSQL, you can experiment with other engines:

```yaml
sql:
  - schema: schema.sql
    queries: queries.sql
    engine: mysql  # or sqlite
    codegen:
      - plugin: rust
        out: src/db
```

### Multiple Database Configurations

You can generate code for multiple databases:

```yaml
version: '2'
plugins:
  - name: rust
    process:
      cmd: sqlc-gen-rust

sql:
  - schema: users_schema.sql
    queries: users_queries.sql
    engine: postgresql
    codegen:
      - plugin: rust
        out: src/db/users
        options:
          package: users

  - schema: products_schema.sql
    queries: products_queries.sql
    engine: postgresql
    codegen:
      - plugin: rust
        out: src/db/products
        options:
          package: products
```

### Custom Binary Path

If you've installed the binary in a custom location:

```yaml
plugins:
  - name: rust
    process:
      cmd: /usr/local/bin/sqlc-gen-rust
      # or use absolute path
      # cmd: /home/user/.cargo/bin/sqlc-gen-rust
```

### Integration with Build Systems

#### Using with Cargo Build Scripts

Add to your `build.rs`:

```rust
use std::process::Command;

fn main() {
    // Generate code during build
    let output = Command::new("sqlc")
        .arg("generate")
        .output()
        .expect("Failed to run sqlc generate");
    
    if !output.status.success() {
        panic!("sqlc generate failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    println!("cargo:rerun-if-changed=schema.sql");
    println!("cargo:rerun-if-changed=queries.sql");
    println!("cargo:rerun-if-changed=sqlc.yaml");
}
```

#### Using with Makefile

```makefile
.PHONY: generate
generate:
	sqlc generate

.PHONY: build
build: generate
	cargo build

.PHONY: test
test: generate
	cargo test
```

## Development

To contribute to this plugin:

1. Clone the repository
2. Run `cargo test` to run tests
3. Build with `cargo build --release`
4. Test with the example: `cd example && sqlc generate`

## License

MIT License