use sqlx::{Postgres, Pool};


#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create_user(pool: &Pool<Postgres>, username: String, email: String, password: String) -> Result<User, sqlx::Error>  {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *",
            username,
            email,
            password
        ).fetch_one(pool).await?;

        Ok(user)
    }

    pub async fn get_user_by_username(pool: &Pool<Postgres>, username: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            username,
        ).fetch_optional(pool).await?;

        match user {
            Some(user) => {
                println!("found user: {:?}", user);
                Ok(user)
            }
            None => {
                println!("found user: {:?}", user);
                Err(sqlx::Error::RowNotFound)
            }
        }
    }
}


#[derive(Debug, serde::Deserialize)]
pub struct RegisterUserSchema {
    pub username: String,
    pub password: String,
}
