use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    println!("🔄 Attempting to connect to database...");
    println!("Database URL: {}", database_url);
    
    // Test connection with more detailed error handling
    match PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Database connection successful!");
            
            // Test a simple query
            match sqlx::query("SELECT 1 as test").fetch_one(&pool).await {
                Ok(_) => println!("✅ Database query test successful!"),
                Err(e) => println!("❌ Database query failed: {}", e),
            }
            
            pool.close().await;
        }
        Err(e) => {
            println!("❌ Database connection failed: {}", e);
            println!("\n🔍 Common solutions:");
            println!("1. Make sure PostgreSQL is running");
            println!("2. Check if the database 'postgres' exists");
            println!("3. Verify username/password are correct");
            println!("4. Ensure the database is accessible on localhost:5432");
        }
    }
    
    Ok(())
}
