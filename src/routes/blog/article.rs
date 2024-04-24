use std::sync::Arc;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::response::Response;
use crate::app_state::AppState;

pub async fn article(
    State(state): State<Arc<AppState<'_>>>,
    Path(id): Path<u32>,
) -> Response {

    let mut path = "./static/entries/".to_string();
    path.push_str(&id.to_string());
    path.push_str(".md");

    let file_content = tokio::fs::read_to_string(path).await;

    if let Err(e) = file_content {

        return Response::builder()
            .status(404)
            .header("Content-Type", "text/html")
            .body(Body::from("WHAT?"))
            .unwrap()

    }

    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::from("Hello!"))
        .unwrap()

}
