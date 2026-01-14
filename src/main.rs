use rust_rsshub::router;
use rust_rsshub::plugins::PluginRegistry;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    // 获取配置目录
    let configs_dir = std::env::var("CONFIGS_DIR")
        .unwrap_or_else(|_| "configs".to_string());

    info!("Loading plugins from: {}", configs_dir);

    // 创建插件注册表
    let registry = Arc::new(PluginRegistry::new(configs_dir)?);

    // 创建路由
    let app = router::create_router(registry);

    // 启动服务器
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()?;

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Server listening on: http://{}", addr);
    info!("Available routes:");
    info!("  GET  /              - Welcome page");
    info!("  GET  /health        - Health check");
    info!("  GET  /plugins       - List all plugins");
    info!("  GET  /rss/:name     - Get RSS feed");

    axum::serve(listener, app).await?;

    Ok(())
}
