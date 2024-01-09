use crate::AppState;

use axum::{
    routing::{get, post},
    Router,
};

use crate::handler::{
        //register_user_handler,
        hello_handler,
        handle_registration_get,
        handle_registration_post,
    };

pub enum ApiError {
    BasRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
}

pub fn init_router(_state: AppState) -> Router {
    Router::new()
        .route("/api/hello", get(hello_handler))
        .route("/api/register", get(handle_registration_get).post(handle_registration_post))
        //.route("/api/auth/register", post(register_user_handler))
}
