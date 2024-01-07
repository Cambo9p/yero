mod user_api;
use crate::AppState;
use std::sync::Arc;
use axum::{response::Html, routing::get, Router};
use crate::templates::Index;
use askama::Template;

pub fn make_app(state: AppState) -> Router<AppState> {
    let user_routes = user_api::user_api(state);

    Router::new()
        .nest("/users", user_routes)
        .route("/", get(index))
}

async fn index() -> Html<String> {
    return Html(Index {}.render().unwrap())
}


