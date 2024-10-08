use crate::{
    api::controllers::{todo_handler, user_handler},
    container::Container,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub fn create_app(container: Arc<Container>) -> Router<()> {
    let app = Router::new()
        .route("/todo", post(todo_handler::create_todo_handler))
        .route("/todo", get(todo_handler::list_todo_handler))
        .route("/todo/:todo_id", get(todo_handler::get_todo_handler))
        .route("/todo/:todo_id", delete(todo_handler::delete_todo_handler))
        .route("/todo/:todo_id", put(todo_handler::completed_todo_handler))
        .route("/user", post(user_handler::register_handler))
        .route("/user/login", post(user_handler::login))
        .layer(TraceLayer::new_for_http())
        .with_state(container);

    app
}
