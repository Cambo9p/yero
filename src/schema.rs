use sqlx::postgres::PgQueryResult;
use sqlx::{Pool,Postgres};

async fn create_user_table(pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL,
            password VARCHAR(255) NOT NULL
            )
        "#;

    sqlx::query(query).execute(pool).await
}

pub async fn create_schema(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    create_user_table(pool).await?; // using ? will return early 
    Ok(())
}
