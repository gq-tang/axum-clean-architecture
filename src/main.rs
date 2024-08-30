use std::sync::Arc;

use axum_clean_architecture::{container::Container, create_app};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let container = Container::new().await;
    let app = create_app::create_app(Arc::new(container));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("now listening on: {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
