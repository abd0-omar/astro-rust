use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use rinja_axum::Template;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .fallback_service(ServeDir::new("frontend/dist"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct NameTemplate<'a> {
    name: &'a str,
}

async fn handler() -> impl IntoResponse {
    Html(
        NameTemplate {
            name: "Travis Scott",
        }
        .render()
        .unwrap(),
    )
}
