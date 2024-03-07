use crate::AppState;

use axum::{
    routing::get,
    Router,
};

use crate::handler::{
        //register_user_handler,
        hello_handler,
        handle_registration_get,
        handle_registration_post,
        handle_home_get,
    };

/*pub enum ApiError {
    BasRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
}*/

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/api/hello", get(hello_handler))
        .route("/register", get(handle_registration_get).post(handle_registration_post))
        .route("/home", get(handle_home_get).post(handle_home_get))
        .with_state(state)
        //.route("/api/auth/register", post(register_user_handler))
}
