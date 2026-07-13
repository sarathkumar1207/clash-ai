pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn app() -> Router {
    presentation::routes()
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
