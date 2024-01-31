use axum::{routing::get, Router};
use tower_http::services::ServeFile;

mod componenets;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::index))
        .route_service("/javascript/htmx", ServeFile::new("static/js/htmx.min.js"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

