use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::error::RssHubError;
use crate::generator::{RssGenerator, AtomGenerator};

#[derive(Clone)]
pub struct AppState {
    pub plugin_registry: Arc<crate::plugins::PluginRegistry>,
}

#[derive(Debug, Deserialize)]
pub struct RssQuery {
    #[serde(default)]
    format: Option<String>,
}

/// RSS 订阅处理器
pub async fn get_rss(
    State(state): State<AppState>,
    Path(plugin_name): Path<String>,
    Query(query): Query<RssQuery>,
) -> Result<Response, RssHubError> {
    // 获取插件配置
    let config = state.plugin_registry.get_plugin(&plugin_name).await?;

    // 执行插件
    let feed = state.plugin_registry.execute_plugin(&config).await?;

    // 根据 format 返回不同格式
    let content = match query.format.as_deref() {
        Some("atom") => AtomGenerator::generate(&feed),
        _ => RssGenerator::generate(&feed),
    };

    Ok((
        [(axum::http::header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        content,
    ).into_response())
}

/// 列出所有插件
pub async fn list_plugins(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, RssHubError> {
    let plugins = state.plugin_registry.list_plugins().await?;
    Ok(Json(plugins))
}

/// 健康检查
pub async fn health_check() -> &'static str {
    "OK"
}

/// 欢迎页面
pub async fn index() -> &'static str {
    r#"
    <html>
    <head><title>Rust RSSHub</title></head>
    <body>
        <h1>Welcome to Rust RSSHub</h1>
        <p>A lightweight RSS feed generator with plugin support</p>
        <ul>
            <li><a href="/health">Health Check</a></li>
            <li><a href="/plugins">List Plugins</a></li>
        </ul>
        <p>Usage: <code>/rss/:plugin_name</code></p>
    </body>
    </html>
    "#
}

impl IntoResponse for RssHubError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RssHubError::PluginNotFound(name) => {
                (StatusCode::NOT_FOUND, format!("Plugin '{}' not found", name))
            }
            RssHubError::InvalidUrl(url) => {
                (StatusCode::BAD_REQUEST, format!("Invalid URL: {}", url))
            }
            RssHubError::Config(msg) => {
                (StatusCode::BAD_REQUEST, format!("Configuration error: {}", msg))
            }
            RssHubError::HttpError(err) => {
                (StatusCode::BAD_GATEWAY, format!("HTTP error: {}", err))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let result = health_check().await;
        assert_eq!(result, "OK");
    }

    #[tokio::test]
    async fn test_index() {
        let result = index().await;
        assert!(result.contains("Rust RSSHub"));
    }
}
