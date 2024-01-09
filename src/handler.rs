use crate::{
    templates::{
        Index,
        Register,
    },
    AppState,
    models::User,
};

use askama::Template;

use axum::{
    extract::State,
    response::Html,
    Form,
};

pub async fn hello_handler() -> Html<String> { 
    return Html(Index {}.render().unwrap());
}

pub async fn handle_registration_get() -> Html<String> { 
    return Html(Register {}.render().unwrap());
}

#[derive(Debug, serde::Deserialize)]
pub struct RegistrationFormData {
    username: String,
    email: String,
    password: String,
}

pub async fn handle_registration_post(State(app_state): State<AppState>,
    Form(form_data): Form<RegistrationFormData>) -> Html<String> { 

    println!("got user: {:?}, pass {:?}", form_data.username, form_data.password);
    let res = User::create_user(&app_state.db, form_data.username, form_data.email, form_data.password).await;

    // TODO: add to template such that it takes an error option and can display that 
    // e.g {{ if err }} 
    match res {
        Ok(_) => {
            // TODO: log the user in
            return Html(Index {}.render().unwrap());
        },
        Err(err) => {
            // TODO: handle all error cases -> what if the user already exists?
            println!("error {}", err);
            return Html(Register {}.render().unwrap());
        }
    }


}
