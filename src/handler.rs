use crate::templates::{
    Index,
    Register,
};

use askama::Template;

use axum::{
    response::Html,
    Form,
};

/* pub async fn register_user_handler(
    Json(body): Json<RegisterUserSchema>) -> Html<String> {
    println!("hit create");
    // TODO fix getting data from form 
    println!("creating{}, {}", body.username, body.password);
    //let _ = User::create_user(&state.db, username.to_string(), password.to_string()).await;


} */

pub async fn hello_handler() -> Html<String> { 
    return Html(Index {}.render().unwrap());
}

pub async fn handle_registration_get() -> Html<String> { 
    return Html(Register {}.render().unwrap());
}

#[derive(Debug, serde::Deserialize)]
pub struct RegistrationFormData {
    username: String,
    password: String,
}

pub async fn handle_registration_post(Form(form_data): Form<RegistrationFormData>) -> Html<String> { 
    println!("got user: {:?}, pass {:?}", form_data.username, form_data.password);
    return Html(Index {}.render().unwrap());
}
