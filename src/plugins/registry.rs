use crate::error::{Result, RssHubError};
use crate::config::types::{PluginConfig, RssFeed};
use crate::config::ConfigParser;
use crate::fetcher::HttpFetcher;
use crate::parser::HtmlParser;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::SystemTime;

pub struct PluginRegistry {
    config_parser: ConfigParser,
    http_fetcher: Arc<HttpFetcher>,
    plugin_cache: Arc<RwLock<std::collections::HashMap<String, CachedPlugin>>>,
}

struct CachedPlugin {
    config: PluginConfig,
    loaded_at: SystemTime,
}

impl PluginRegistry {
    pub fn new(configs_dir: String) -> Result<Self> {
        Ok(Self {
            config_parser: ConfigParser::new(configs_dir),
            http_fetcher: Arc::new(HttpFetcher::new()?),
            plugin_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }

    /// 获取插件配置(带缓存和自动刷新)
    pub async fn get_plugin(&self, name: &str) -> Result<PluginConfig> {
        // 检查插件是否存在
        if !self.config_parser.plugin_exists(name) {
            return Err(RssHubError::PluginNotFound(name.to_string()));
        }

        // 检查缓存
        let current_mtime = self.config_parser.get_plugin_mtime(name)?;
        let cache = self.plugin_cache.read().await;

        if let Some(cached) = cache.get(name) {
            // 对比修改时间
            if cached.loaded_at >= current_mtime {
                return Ok(cached.config.clone());
            }
        }
        drop(cache);

        // 加载配置
        let config = self.config_parser.load_plugin(name)?;

        // 更新缓存
        let mut cache = self.plugin_cache.write().await;
        cache.insert(name.to_string(), CachedPlugin {
            config: config.clone(),
            loaded_at: SystemTime::now(),
        });

        Ok(config)
    }

    /// 执行插件
    pub async fn execute_plugin(&self, config: &PluginConfig) -> Result<RssFeed> {
        // 获取内容
        let html = self.http_fetcher.fetch_html(&config.source).await?;

        // 提取基础URL用于处理相对链接
        let base_url = match &config.source {
            crate::config::types::SourceConfig::Html(cfg) => &cfg.url,
            crate::config::types::SourceConfig::Json(cfg) => &cfg.url,
            crate::config::types::SourceConfig::Xml(cfg) => &cfg.url,
        };

        // 解析
        let articles = HtmlParser::parse(&html, &config.parser, base_url, config.feed.limit)?;

        // 构建 Feed
        let feed = RssFeed {
            title: config.feed.title.clone(),
            description: config.feed.description.clone(),
            link: config.feed.link.clone(),
            language: config.feed.language.clone(),
            articles,
        };

        Ok(feed)
    }

    /// 列出所有插件
    pub async fn list_plugins(&self) -> Result<Vec<String>> {
        self.config_parser.list_plugins()
    }

    /// 清除插件缓存
    pub async fn invalidate_plugin(&self, name: &str) {
        let mut cache = self.plugin_cache.write().await;
        cache.remove(name);
    }

    /// 清除所有缓存
    pub async fn invalidate_all(&self) {
        let mut cache = self.plugin_cache.write().await;
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let temp_dir = std::env::temp_dir();
        let registry = PluginRegistry::new(temp_dir.to_str().unwrap().to_string());
        assert!(registry.is_ok());
    }

    #[tokio::test]
    async fn test_plugin_not_found() {
        let temp_dir = std::env::temp_dir();
        let registry = PluginRegistry::new(temp_dir.to_str().unwrap().to_string()).unwrap();
        let result = registry.get_plugin("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_plugins() {
        let temp_dir = std::env::temp_dir();
        let registry = PluginRegistry::new(temp_dir.to_str().unwrap().to_string()).unwrap();
        let plugins = registry.list_plugins().await;
        assert!(plugins.is_ok());
    }

    #[tokio::test]
    async fn test_invalidate_plugin() {
        let temp_dir = std::env::temp_dir();
        let registry = PluginRegistry::new(temp_dir.to_str().unwrap().to_string()).unwrap();
        registry.invalidate_plugin("test").await;
        // Should not panic
    }

    #[tokio::test]
    async fn test_invalidate_all() {
        let temp_dir = std::env::temp_dir();
        let registry = PluginRegistry::new(temp_dir.to_str().unwrap().to_string()).unwrap();
        registry.invalidate_all().await;
        // Should not panic
    }
}
