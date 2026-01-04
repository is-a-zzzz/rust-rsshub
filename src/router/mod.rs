pub mod handlers;

pub use handlers::{AppState, index, health_check, list_plugins, get_rss};

use axum::{
    Router,
    routing::get,
};
use std::sync::Arc;
use crate::plugins::PluginRegistry;

pub fn create_router(registry: Arc<PluginRegistry>) -> Router {
    let state = handlers::AppState {
        plugin_registry: registry,
    };

    Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/plugins", get(list_plugins))
        .route("/rss/:plugin_name", get(get_rss))
        .with_state(state)
}
