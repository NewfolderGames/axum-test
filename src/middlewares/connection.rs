use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn connection(request: Request, next: Next) -> Response {

    // let uri =  request.uri().to_string();
    let response = next.run(request).await;

    response

}
