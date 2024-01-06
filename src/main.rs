mod templates;
use axum::{response::Html, routing::get, Router};
use templates::Index;
use askama::Template;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<String> {
    return Html(Index {}.render().unwrap())
}
