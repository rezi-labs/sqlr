mod db;

use sqlx::postgres::PgPool;
use db::Database;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // This is a sample main function - in a real app you'd connect to a database
    println!("Example sqlc-gen-rust generated code");
    println!("To use:");
    println!("1. Set up a PostgreSQL database");
    println!("2. Run: let pool = PgPool::connect(\"postgresql://...\").await?;");
    println!("3. Run: let db = Database::new(pool);");
    println!("4. Use the generated query methods like: db.get_user(1).await?");
    
    Ok(())
}