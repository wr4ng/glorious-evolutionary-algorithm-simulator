use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tower_http::cors::CorsLayer;

mod eas;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/tasks", get(get_tasks))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_tasks() -> (StatusCode, Json<Vec<Task>>) {
    let task = vec![
        Task {
            id: "some-id".to_string(),
        },
        Task {
            id: "other-id".to_string(),
        },
    ];
    (StatusCode::OK, Json(task))
}

#[derive(Serialize)]
struct Task {
    id: String,
}
