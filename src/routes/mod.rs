pub mod blog;

use axum::body::Body;
use axum::response::{Response};

pub async fn hello_world() -> Response {

    Response::builder()
        .status(200)
        .header("X-Hello-World", "Hello, world!")
        .body(Body::from("Hello, world!"))
        .unwrap()

}
