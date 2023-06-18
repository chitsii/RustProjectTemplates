use std::collections::HashMap;
use std::sync::Arc;
use std::{hash::Hash, net::SocketAddr};

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let app = Router::new()
        .route("/", get(root))
        .route("/hc", get(health_check))
        .route("/echo/:id", get(return_value));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn root() -> Html<&'static str> {
    Html("<h1>OK</h1>")
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn return_value(Path(id): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let res: HashMap<String, String> = HashMap::from_iter([("id".to_string(), id)]);

    if res.get("id").unwrap() == "error" {
        Err(StatusCode::BAD_REQUEST)
    } else {
        Ok((StatusCode::OK, Json(res)))
    }
}
