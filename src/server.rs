use std::io::Error;

use axum::{
    Json, Router,
    response::{Html, IntoResponse},
    routing::{get, get_service},
};
use tokio::fs;
use tower_http::services::ServeDir;

#[tokio::main]
pub async fn run() -> Result<(), Error> {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/notes", get(list_notes))
        .nest_service(
            "/notes",
            get_service(ServeDir::new("./public/notes")).handle_error(|err| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Static file error: {}", err),
                )
            }),
        )
        .fallback_service(ServeDir::new("./public"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn index() -> impl IntoResponse {
    match fs::read_to_string("./public/index.html").await {
        Ok(content) => Html(content).into_response(),
        Err(_) => Html("<h1>Internal Server Error</h1>").into_response(),
    }
}

pub async fn list_notes() -> impl IntoResponse {
    let mut entries = match fs::read_dir("./public/notes/test").await {
        Ok(p) => p,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read notes directory: {}", e),
            )
                .into_response();
        }
    };
    let mut files = Vec::new();

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|x| x.to_str()) {
                files.push(name.to_string());
            }
        }
    }

    Json(files).into_response()
}
