mod schema;
mod route;
mod models;
mod templates;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use std::sync::Arc;
use route::make_app;
use schema::create_schema;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    match create_schema(&pool).await {
        Ok(_) => println!("✅ Database schema created successfully!"),
        Err(err) => println!("🔥 Failed to create database schema: {:?}", err),
    }

    let app = make_app(Arc::new(AppState { db: pool.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
