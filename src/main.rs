use std::sync::Arc;
use axum::middleware::from_fn;
use axum::Router;
use axum::routing::get;
use handlebars::Handlebars;

mod routes;
mod middlewares;
mod utils;
mod app_state;

#[tokio::main]
async fn main() {

    utils::log::info("Loading handlebar files.");

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("base", "./static/templates/base.hbs").unwrap();

    utils::log::info("Creating app.");
    
    let app_state = Arc::new(app_state::AppState{
        handlebars
    });

    let app = Router::new()
        .route("/helloworld", get(routes::hello_world))
        .route("/blog/article/:id", get(routes::blog::article))
        .layer(from_fn(middlewares::connection))
        .with_state(app_state);

    utils::log::info("Creating listener.");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:10114")
        .await
        .unwrap();

    utils::log::info("Starting app.");

    axum::serve(listener, app).await.unwrap();

}
