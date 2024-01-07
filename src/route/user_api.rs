use std::sync::Arc;

use crate::AppState;
use crate::templates::Register;
use askama::Template;
use axum::{
    response::Html,
    extract::Form,
    Router,
    routing::get, routing::post,
    extract::State,
};
use crate::models::User;


#[derive(serde::Deserialize)]
struct CreateUserForm {
    username: String,
    password: String,
}

pub fn user_api(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/register", get(register))
        .route("/createAccount", post(handle_user_sign_up)).with_state(state.clone())
}

async fn register() -> Html<String> {
    return Html(Register {}.render().unwrap())
}

async fn handle_user_sign_up(Form(sign_up): Form<CreateUserForm>, State(state): State<AppState>) -> Html<String> {
    println!("hit create");
    let username = &sign_up.username;
    let password = &sign_up.password;
    println!("Creating user with username: {} and password: {}", username, password);
    User::create_user(&state.db, username.to_string(), password.to_string());



    Html(format!("User created with username: {}", username))
}
