use crate::{ templates::{
        Home,
        Register,
    },
    AppState,
    models::User,
    auth::{self},
};

use askama::Template;

use axum::{
    extract::State,
    response::{Redirect, IntoResponse, Html, Json},
    Form,
    http::{StatusCode, HeaderMap, Response, header::AUTHORIZATION},
};

use lettre::transport::smtp::commands::Auth;
use serde::Serialize;

#[derive(Serialize)]
struct RedirectResponseData {
    redirect_url: String,
    jwt_token: String,
}

pub async fn hello_handler() -> Html<String> { 
    return Html(Home {}.render().unwrap());
}

pub async fn handle_registration_get() -> Html<String> { 
    println!("GET /register");
    return Html(Register {}.render().unwrap());
}

#[derive(Debug, serde::Deserialize)]
pub struct RegistrationFormData {
    username: String,
    email: String,
    password: String,
}

// redirects to home when the user is registered
pub async fn handle_registration_post(
    State(app_state): State<AppState>,
    Form(form_data): Form<RegistrationFormData>
    ) -> impl IntoResponse { 

    println!("POST /register");
    println!("got user: {:?}, pass {:?}", form_data.username.to_owned(), form_data.password);
    let res = User::create_user(&app_state.db, form_data.username.clone(), form_data.email.clone(), form_data.password.clone()).await;

    // TODO: add to template such that it takes an error option and can display that 
    // e.g {{ if err }} 
    match res {
        Ok(_) => {
            println!("got to right spot");
            
            let jwt_token = auth::generate_token(form_data.username);
            let redirect_url = "/home"; // Assuming this is your redirect URL

            axum::http::Response::builder()
                .header(AUTHORIZATION, jwt_token)
                .body(redirect_url.into())
                .unwrap()
        },
        Err(err) => {
            // TODO: handle all error cases -> what if the user already exists?
            println!("error {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...",).into_response()
        }
    }
}

pub async fn handle_home_get(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    ) -> impl IntoResponse { 
    println!("GET /home");
    // TODO json is sent but need to display html 

    match auth::validate_token(&headers, &app_state.jwt_secret) {
        Ok(_) => {
            return Html(Home {}.render().unwrap());
        }
        Err(_) => {
            return Html(Register {}.render().unwrap());
        }
    }
}
