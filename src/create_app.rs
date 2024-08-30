use crate::{
    api::{controllers::todo_handler, dto::todo},
    container::Container,
};
use axum::{routing::post, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub fn create_app(container: Arc<Container>) -> Router<()> {
    let todo_service = container.todo_service.clone();
    let app = Router::new()
        .route("/todo", post(todo_handler::create))
        .layer(TraceLayer::new_for_http())
        .with_state(todo_service);
    app
}
