use crate::{api::controllers::todo_handler, container::Container};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub fn create_app(container: Arc<Container>) -> Router<()> {
    let todo_service = container.todo_service.clone();
    let app = Router::new()
        .route("/todo", post(todo_handler::create))
        .route("/todo", get(todo_handler::list))
        .layer(TraceLayer::new_for_http())
        .with_state(todo_service);
    app
}
