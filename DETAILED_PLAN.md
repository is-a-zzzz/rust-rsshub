# Rust RSSHub 详细实施计划

## 目录
1. [项目初始化](#第一阶段-项目初始化)
2. [核心数据结构定义](#第二阶段-核心数据结构定义)
3. [配置系统实现](#第三阶段-配置系统实现)
4. [HTTP 客户端和内容获取](#第四阶段-http-客户端和内容获取)
5. [解析器系统实现](#第五阶段-解析器系统实现)
6. [RSS 生成器实现](#第六阶段-rss-生成器实现)
7. [Web 服务器和路由](#第七阶段-web-服务器和路由)
8. [插件系统核心](#第八阶段-插件系统核心)
9. [示例配置和测试](#第九阶段-示例配置和测试)
10. [Docker 部署配置](#第十阶段-docker-部署配置)

---

## 第一阶段: 项目初始化

### 1.1 创建 Cargo 项目

**操作步骤:**
```bash
cd /root/rust-rsshub
cargo init --name rust-rsshub
```

### 1.2 配置 Cargo.toml

**文件:** `Cargo.toml`

**完整配置:**
```toml
[package]
name = "rust-rsshub"
version = "0.1.0"
edition = "2021"
authors = ["Your Name"]

[dependencies]
# 异步运行时
tokio = { version = "1.40", features = ["full"] }

# Web 框架
axum = { version = "0.7", features = ["multipart"] }
tower = "0.5"
tower-http = { version = "0.5", features = ["cors", "trace", "compression"] }

# HTTP 客户端
reqwest = { version = "0.12", features = ["json", "cookies"] }

# HTML 解析
scraper = "0.20"
select = "0.6"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# RSS/Atom 生成
rss = "2.0"
atom_syndication = "0.12"

# 日期处理
chrono = { version = "0.4", features = ["serde"] }

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 配置和环境变量
config = "0.14"

# 文件系统监控(可选)
notify = { version = "6.1", optional = true }

# URL 处理
url = "2.5"

# 编码检测
encoding_rs = "0.8"

[dev-dependencies]
tokio-test = "0.4"

[features]
default = ["file-watch"]
file-watch = ["notify"]

[[bin]]
name = "rust-rsshub"
path = "src/main.rs"
```

### 1.3 创建目录结构

**操作步骤:**
```bash
mkdir -p src/{config,router,fetcher,parser,generator,plugins}
mkdir -p configs
mkdir -p docs
mkdir -p tests
touch configs/.keep
```

**目录结构:**
```
rust-rsshub/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   └── types.rs
│   ├── router/
│   │   ├── mod.rs
│   │   └── handlers.rs
│   ├── fetcher/
│   │   ├── mod.rs
│   │   ├── http.rs
│   │   └── cache.rs
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── html.rs
│   │   ├── json.rs
│   │   └── selector.rs
│   ├── generator/
│   │   ├── mod.rs
│   │   ├── rss.rs
│   │   └── atom.rs
│   ├── plugins/
│   │   ├── mod.rs
│   │   ├── registry.rs
│   │   ├── loader.rs
│   │   └── executor.rs
│   └── error.rs
├── configs/
│   └── .keep
├── docs/
├── tests/
├── Dockerfile
├── docker-compose.yml
└── README.md
```

---

## 第二阶段: 核心数据结构定义

### 2.1 错误类型定义

**文件:** `src/error.rs`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RssHubError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Plugin '{0}' not found")]
    PluginNotFound(String),

    #[error("Invalid YAML: {0}")]
    InvalidYaml(#[from] serde_yaml::Error),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

pub type Result<T> = std::result::Result<T, RssHubError>;
```

### 2.2 配置类型定义

**文件:** `src/config/types.rs`

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 插件配置根节点
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginConfig {
    pub plugin: PluginMetadata,
    pub source: SourceConfig,
    pub parser: ParserConfig,
    #[serde(default)]
    pub cache: CacheConfig,
    pub feed: FeedConfig,
}

/// 插件元数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginMetadata {
    pub name: String,
    pub description: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub author: Option<String>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

/// 数据源配置
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SourceConfig {
    #[serde(rename = "html")]
    Html(HtmlSourceConfig),
    #[serde(rename = "json")]
    Json(JsonSourceConfig),
    #[serde(rename = "xml")]
    Xml(XmlSourceConfig),
}

/// HTML 数据源配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HtmlSourceConfig {
    pub url: String,
    #[serde(default = "default_encoding")]
    pub encoding: String,
    #[serde(default)]
    pub user_agent: Option<String>,
    #[serde(default)]
    pub request: RequestConfig,
}

fn default_encoding() -> String {
    "utf-8".to_string()
}

/// JSON 数据源配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonSourceConfig {
    pub url: String,
    #[serde(default)]
    pub request: RequestConfig,
}

/// XML 数据源配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XmlSourceConfig {
    pub url: String,
    #[serde(default)]
    pub request: RequestConfig,
}

/// HTTP 请求配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestConfig {
    #[serde(default = "default_method")]
    pub method: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            method: default_method(),
            headers: HashMap::new(),
            timeout: default_timeout(),
        }
    }
}

fn default_method() -> String {
    "GET".to_string()
}

fn default_timeout() -> u64 {
    30
}

/// 解析器配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParserConfig {
    pub list: ListParserConfig,
    #[serde(default)]
    pub content: Option<ContentParserConfig>,
}

/// 列表页解析配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListParserConfig {
    pub selector: String,
    #[serde(default)]
    pub item_selector: Option<String>,
    pub link_selector: String,
    pub title_selector: String,
    #[serde(default)]
    pub description_selector: Option<String>,
    #[serde(default)]
    pub date_selector: Option<String>,
    #[serde(default)]
    pub date_format: Option<String>,
    #[serde(default)]
    pub author_selector: Option<String>,
    #[serde(default)]
    pub category_selector: Option<String>,
}

/// 内容页解析配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentParserConfig {
    pub selector: String,
    pub content_selector: String,
    #[serde(default)]
    pub cleanup_selectors: Vec<String>,
}

/// 缓存配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    #[serde(default = "default_cache_enabled")]
    pub enabled: bool,
    #[serde(default = "default_cache_ttl")]
    pub ttl: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: default_cache_enabled(),
            ttl: default_cache_ttl(),
        }
    }
}

fn default_cache_enabled() -> bool {
    true
}

fn default_cache_ttl() -> u64 {
    3600
}

/// Feed 配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeedConfig {
    pub title: String,
    pub description: String,
    pub link: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_feed_format")]
    pub format: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_language() -> String {
    "en".to_string()
}

fn default_feed_format() -> String {
    "rss".to_string()
}

fn default_limit() -> usize {
    20
}

/// 解析后的文章条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub link: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub pub_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub category: Option<String>>,
    #[serde(default)]
    pub guid: Option<String>,
}

/// RSS Feed 数据
#[derive(Debug, Clone)]
pub struct RssFeed {
    pub title: String,
    pub description: String,
    pub link: String,
    pub language: String,
    pub articles: Vec<Article>,
}
```

### 2.3 在 lib.rs 中导出类型

**文件:** `src/lib.rs`

```rust
pub mod error;
pub mod config;
pub mod router;
pub mod fetcher;
pub mod parser;
pub mod generator;
pub mod plugins;

pub use error::{RssHubError, Result};
pub use config::types::*;
```

---

## 第三阶段: 配置系统实现

### 3.1 配置解析器

**文件:** `src/config/parser.rs`

```rust
use crate::error::Result;
use crate::config::types::PluginConfig;
use std::path::Path;
use std::fs;
use std::time::SystemTime;

pub struct ConfigParser {
    configs_dir: String,
}

impl ConfigParser {
    pub fn new(configs_dir: impl Into<String>) -> Self {
        Self {
            configs_dir: configs_dir.into(),
        }
    }

    /// 检查插件配置文件是否存在
    pub fn plugin_exists(&self, name: &str) -> bool {
        let path = self.get_config_path(name);
        path.exists()
    }

    /// 获取配置文件路径
    pub fn get_config_path(&self, name: &str) -> std::path::PathBuf {
        Path::new(&self.configs_dir).join(format!("{}.yml", name))
    }

    /// 获取配置文件修改时间
    pub fn get_plugin_mtime(&self, name: &str) -> Result<SystemTime> {
        let path = self.get_config_path(name);
        let metadata = fs::metadata(&path)?;
        Ok(metadata.modified()?)
    }

    /// 加载并解析插件配置
    pub fn load_plugin(&self, name: &str) -> Result<PluginConfig> {
        let path = self.get_config_path(name);
        let content = fs::read_to_string(&path)?;

        // 解析 YAML
        let config: PluginConfig = serde_yaml::from_str(&content)?;

        // 验证配置
        self.validate_config(&config)?;

        Ok(config)
    }

    /// 验证配置有效性
    fn validate_config(&self, config: &PluginConfig) -> Result<()> {
        // 验证必需字段
        if config.feed.title.is_empty() {
            return Err(RssHubError::Config("Feed title cannot be empty".into()));
        }

        if config.feed.link.is_empty() {
            return Err(RssHubError::Config("Feed link cannot be empty".into()));
        }

        // 验证 URL 格式
        let url = match &config.source {
            SourceConfig::Html(cfg) => &cfg.url,
            SourceConfig::Json(cfg) => &cfg.url,
            SourceConfig::Xml(cfg) => &cfg.url,
        };

        if url.parse::<url::Url>().is_err() {
            return Err(RssHubError::InvalidUrl(url.clone()));
        }

        Ok(())
    }

    /// 列出所有可用的插件
    pub fn list_plugins(&self) -> Result<Vec<String>> {
        let dir = fs::read_dir(&self.configs_dir)?;
        let mut plugins = Vec::new();

        for entry in dir {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yml" || ext == "yaml" {
                        if let Some(stem) = path.file_stem() {
                            if let Some(name) = stem.to_str() {
                                if !name.starts_with('.') {
                                    plugins.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        plugins.sort();
        Ok(plugins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path() {
        let parser = ConfigParser::new("/tmp/configs");
        let path = parser.get_config_path("test");
        assert_eq!(path, PathBuf::from("/tmp/configs/test.yml"));
    }
}
```

### 3.2 配置模块入口

**文件:** `src/config/mod.rs`

```rust
pub mod types;
pub mod parser;

pub use types::*;
pub use parser::ConfigParser;
```

---

## 第四阶段: HTTP 客户端和内容获取

### 4.1 HTTP 客户端实现

**文件:** `src/fetcher/http.rs`

```rust
use crate::error::{Result, RssHubError};
use crate::config::types::{SourceConfig, RequestConfig};
use reqwest::Client;
use std::time::Duration;
use encoding_rs::UTF_8;

pub struct HttpFetcher {
    client: Client,
}

impl HttpFetcher {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(90))
            .build()?;

        Ok(Self { client })
    }

    pub fn with_timeout(timeout_secs: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()?;

        Ok(Self { client })
    }

    /// 获取 HTML 内容
    pub async fn fetch_html(&self, source: &SourceConfig) -> Result<String> {
        let (url, config) = match source {
            SourceConfig::Html(cfg) => (&cfg.url, &cfg.request),
            SourceConfig::Json(cfg) => (&cfg.url, &cfg.request),
            SourceConfig::Xml(cfg) => (&cfg.url, &cfg.request),
        };

        let mut request = self.client.request(
            reqwest::Method::from_bytes(config.method.as_bytes())?,
            url,
        );

        // 添加 headers
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }

        // 设置超时
        request = request.timeout(Duration::from_secs(config.timeout));

        // 发送请求
        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(RssHubError::HttpError(
                reqwest::Error::from(
                    reqwest::StatusCode::from_u16(response.status().as_u16())
                        .unwrap_or(reqwest::StatusCode::BAD_REQUEST)
                )
            ));
        }

        // 获取响应字节
        let bytes = response.bytes().await?;

        // 检测编码并转换为 UTF-8
        let (html, _, _) = if let SourceConfig::Html(cfg) = source {
            let encoding = encoding_rs::Encoding::for_label(cfg.encoding.as_bytes())
                .unwrap_or(UTF_8);
            encoding.decode(&bytes).into_owned()
        } else {
            UTF_8.decode(&bytes).into_owned()
        };

        Ok(html.to_string())
    }

    /// 获取 JSON 内容
    pub async fn fetch_json(&self, url: &str) -> Result<serde_json::Value> {
        let response = self.client.get(url).send().await?;
        let json = response.json().await?;
        Ok(json)
    }
}

impl Default for HttpFetcher {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP fetcher")
    }
}
```

### 4.2 内存缓存实现

**文件:** `src/fetcher/cache.rs`

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct CacheEntry {
    pub content: String,
    pub created_at: Instant,
}

pub struct MemoryCache {
    entries: RwLock<HashMap<String, CacheEntry>>,
    ttl: Duration,
}

impl MemoryCache {
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    /// 获取缓存内容
    pub async fn get(&self, key: &str) -> Option<String> {
        let entries = self.entries.read().await;
        if let Some(entry) = entries.get(key) {
            if entry.created_at.elapsed() < self.ttl {
                return Some(entry.content.clone());
            }
        }
        None
    }

    /// 设置缓存
    pub async fn set(&self, key: String, content: String) {
        let entry = CacheEntry {
            content,
            created_at: Instant::now(),
        };
        let mut entries = self.entries.write().await;
        entries.insert(key, entry);
    }

    /// 清除过期缓存
    pub async fn cleanup(&self) {
        let mut entries = self.entries.write().await;
        entries.retain(|_, entry| entry.created_at.elapsed() < self.ttl);
    }

    /// 清除指定缓存
    pub async fn invalidate(&self, key: &str) {
        let mut entries = self.entries.write().await;
        entries.remove(key);
    }

    /// 清除所有缓存
    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
    }
}
```

### 4.3 Fetcher 模块入口

**文件:** `src/fetcher/mod.rs`

```rust
pub mod http;
pub mod cache;

pub use http::HttpFetcher;
pub use cache::{MemoryCache, CacheEntry};
```

---

## 第五阶段: 解析器系统实现

### 5.1 HTML 解析器实现

**文件:** `src/parser/html.rs`

```rust
use crate::error::{Result, RssHubError};
use crate::config::types::{ParserConfig, Article};
use scraper::{Html, Selector, ElementRef};
use chrono::DateTime;
use std::time::SystemTime;

pub struct HtmlParser;

impl HtmlParser {
    pub fn parse(html: &str, config: &ParserConfig, base_url: &str) -> Result<Vec<Article>> {
        let document = Html::parse_document(html);

        // 解析列表
        let list_selector = Selector::parse(&config.list.selector)
            .map_err(|e| RssHubError::ParseError(format!("Invalid list selector: {}", e)))?;

        let mut articles = Vec::new();

        for element in document.select(&list_selector) {
            if let Some(article) = Self::parse_article(&element, &config.list, base_url)? {
                articles.push(article);
            }

            // 限制数量
            if articles.len() >= config.list.limit.unwrap_or(20) {
                break;
            }
        }

        Ok(articles)
    }

    fn parse_article(
        element: &ElementRef,
        config: &ListParserConfig,
        base_url: &str,
    ) -> Result<Option<Article>> {
        // 提取标题
        let title = Self::extract_text(element, &config.title_selector)?;

        // 提取链接
        let link = Self::extract_link(element, &config.link_selector, base_url)?;

        // 提取描述
        let description = if let Some(ref desc_selector) = config.description_selector {
            Some(Self::extract_text(element, desc_selector)?)
        } else {
            None
        };

        // 提取日期
        let pub_date = if let Some(ref date_selector) = config.date_selector {
            Self::extract_date(element, date_selector, config.date_format.as_deref())?
        } else {
            None
        };

        // 提取作者
        let author = if let Some(ref author_selector) = config.author_selector {
            Some(Self::extract_text(element, author_selector)?)
        } else {
            None
        };

        Ok(Some(Article {
            title,
            link,
            description,
            content: None,
            pub_date,
            author,
            category: None,
            guid: None,
        }))
    }

    fn extract_text(element: &ElementRef, selector: &str) -> Result<String> {
        let sel = Selector::parse(selector)
            .map_err(|e| RssHubError::ParseError(format!("Invalid selector: {}", e)))?;

        if let Some(el) = element.select(&sel).next() {
            Ok(el.text().collect::<Vec<_>>().join("").trim().to_string())
        } else {
            Ok(String::new())
        }
    }

    fn extract_link(element: &ElementRef, selector: &str, base_url: &str) -> Result<String> {
        let sel = Selector::parse(selector)
            .map_err(|e| RssHubError::ParseError(format!("Invalid selector: {}", e)))?;

        if let Some(el) = element.select(&sel).next() {
            if let Some(href) = el.value().attr("href") {
                // 处理相对链接
                if href.starts_with("http") {
                    Ok(href.to_string())
                } else if href.starts_with("/") {
                    Ok(format!("{}{}", base_url.trim_end_matches('/'), href))
                } else {
                    Ok(format!("{}/{}", base_url.trim_end_matches('/'), href))
                }
            } else {
                Err(RssHubError::ParseError("Link not found".into()))
            }
        } else {
            Err(RssHubError::ParseError("Link element not found".into()))
        }
    }

    fn extract_date(
        element: &ElementRef,
        selector: &str,
        format: Option<&str>,
    ) -> Result<Option<DateTime<chrono::Utc>>> {
        let text = Self::extract_text(element, selector)?;

        if text.is_empty() {
            return Ok(None);
        }

        // 尝试多种日期格式
        if let Some(fmt) = format {
            if let Ok(dt) = chrono::DateTime::parse_from_str(&text, fmt) {
                return Ok(Some(dt.with_timezone(&chrono::Utc)));
            }
        }

        // 尝试常见格式
        let formats = [
            "%B %d, %Y",
            "%Y-%m-%d",
            "%Y-%m-%dT%H:%M:%S%z",
            "%Y-%m-%dT%H:%M:%SZ",
            "%a, %d %b %Y %H:%M:%S %z",
        ];

        for fmt in &formats {
            if let Ok(dt) = chrono::DateTime::parse_from_str(&text, fmt) {
                return Ok(Some(dt.with_timezone(&chrono::Utc)));
            }
        }

        Ok(None)
    }
}
```

### 5.2 Parser 模块入口

**文件:** `src/parser/mod.rs`

```rust
pub mod html;

pub use html::HtmlParser;
```

---

## 第六阶段: RSS 生成器实现

### 6.1 RSS 2.0 生成器

**文件:** `src/generator/rss.rs`

```rust
use crate::config::types::{Article, RssFeed};
use rss::{Channel, ChannelBuilder, Item, ItemBuilder, GuidBuilder};

pub struct RssGenerator;

impl RssGenerator {
    pub fn generate(feed: &RssFeed) -> String {
        let mut channel = ChannelBuilder::default()
            .title(&feed.title)
            .link(&feed.link)
            .description(&feed.description)
            .language(Some(feed.language.clone()))
            .build();

        // 添加文章
        let items: Vec<Item> = feed.articles.iter()
            .map(|article| Self::article_to_item(article))
            .collect();

        channel.items = items;

        channel.to_string()
    }

    fn article_to_item(article: &Article) -> Item {
        let mut item = ItemBuilder::default()
            .title(Some(article.title.clone()))
            .link(Some(article.link.clone()));

        // 描述
        if let Some(ref desc) = article.description {
            item = item.description(Some(desc.clone()));
        }

        // 内容
        if let Some(ref content) = article.content {
            item = item.content(Some(content.clone()));
        }

        // 发布日期
        if let Some(pub_date) = article.pub_date {
            item = item.pub_date(Some(pub_date.to_rfc2822()));
        }

        // 作者
        if let Some(ref author) = article.author {
            item = item.author(Some(author.clone()));
        }

        // GUID
        let guid = GuidBuilder::default()
            .value(article.guid.as_ref().unwrap_or(&article.link).clone())
            .permalink(false)
            .build();
        item = item.guid(Some(guid));

        item.build()
    }
}
```

### 6.2 Atom 1.0 生成器

**文件:** `src/generator/atom.rs`

```rust
use crate::config::types::{Article, RssFeed};
use atom_syndication::{Feed, Entry, EntryBuilder, Link, Person, ContentBuilder};

pub struct AtomGenerator;

impl AtomGenerator {
    pub fn generate(feed: &RssFeed) -> String {
        let mut atom_feed = Feed::default();
        atom_feed.set_title(feed.title.clone());
        atom_feed.set_id(feed.link.clone());
        atom_feed.set_links(vec![Link {
            href: feed.link.clone(),
            rel: "alternate".to_string(),
            mime_type: None,
            title: None,
            hreflang: None,
            length: None,
        }]);

        // 添加文章
        let entries: Vec<Entry> = feed.articles.iter()
            .map(|article| Self::article_to_entry(article))
            .collect();

        atom_feed.set_entries(entries);

        atom_feed.to_string()
    }

    fn article_to_entry(article: &Article) -> Entry {
        let mut entry = EntryBuilder::default();
        entry.title(article.title.clone());

        // ID
        entry.id(article.guid.as_ref().unwrap_or(&article.link).clone());

        // 链接
        entry.links(vec![Link {
            href: article.link.clone(),
            rel: "alternate".to_string(),
            mime_type: None,
            title: None,
            hreflang: None,
            length: None,
        }]);

        // 内容
        if let Some(ref content) = article.content {
            let content = ContentBuilder::default()
                .value(Some(content.clone()))
                .build();
            entry.content(content);
        }

        // 摘要
        if let Some(ref summary) = article.description {
            entry.summary(summary.clone());
        }

        // 发布日期
        if let Some(pub_date) = article.pub_date {
            entry.published(pub_date.to_rfc3339());
        }

        // 作者
        if let Some(ref author) = article.author {
            entry.authors(vec![Person {
                name: author.clone(),
                email: None,
                uri: None,
            }]);
        }

        entry.build()
    }
}
```

### 6.3 Generator 模块入口

**文件:** `src/generator/mod.rs`

```rust
pub mod rss;
pub mod atom;

pub use rss::RssGenerator;
pub use atom::AtomGenerator;
```

---

## 第七阶段: Web 服务器和路由

### 7.1 HTTP 处理器实现

**文件:** `src/router/handlers.rs`

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::config::types::RssFeed;
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
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, message).into_response()
    }
}
```

### 7.2 路由模块入口

**文件:** `src/router/mod.rs`

```rust
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
```

---

## 第八阶段: 插件系统核心

### 8.1 插件注册表

**文件:** `src/plugins/registry.rs`

```rust
use crate::error::{Result, RssHubError};
use crate::config::types::{PluginConfig, RssFeed, Article};
use crate::config::ConfigParser;
use crate::fetcher::{HttpFetcher, MemoryCache};
use crate::parser::HtmlParser;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::SystemTime;

pub struct PluginRegistry {
    config_parser: ConfigParser,
    http_fetcher: HttpFetcher,
    cache: RwLock<MemoryCache>,
    plugin_cache: RwLock<std::collections::HashMap<String, CachedPlugin>>,
}

struct CachedPlugin {
    config: PluginConfig,
    loaded_at: SystemTime,
}

impl PluginRegistry {
    pub fn new(configs_dir: String) -> Result<Self> {
        Ok(Self {
            config_parser: ConfigParser::new(configs_dir),
            http_fetcher: HttpFetcher::new()?,
            cache: RwLock::new(MemoryCache::new(3600)),
            plugin_cache: RwLock::new(std::collections::HashMap::new()),
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
        // 检查缓存
        let cache_key = format!("feed:{}:{}", config.plugin.name,
            chrono::Utc::now().format("%Y%m%d%H"));

        if config.cache.enabled {
            if let Some(cached) = self.cache.read().await.get(&cache_key).await {
                // 返回缓存的 RSS
                return Ok(serde_json::from_str(&cached)?);
            }
        }

        // 获取内容
        let html = self.http_fetcher.fetch_html(&config.source).await?;

        // 解析
        let articles = HtmlParser::parse(&html, &config.parser, &config.feed.link)?;

        // 构建 Feed
        let feed = RssFeed {
            title: config.feed.title.clone(),
            description: config.feed.description.clone(),
            link: config.feed.link.clone(),
            language: config.feed.language.clone(),
            articles,
        };

        // 缓存结果
        if config.cache.enabled {
            let serialized = serde_json::to_string(&feed)?;
            self.cache.write().await.set(cache_key, serialized).await;
        }

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
        self.cache.write().await.clear().await;
    }
}
```

### 8.2 插件模块入口

**文件:** `src/plugins/mod.rs`

```rust
pub mod registry;

pub use registry::PluginRegistry;
```

---

## 第九阶段: 主程序和示例配置

### 9.1 主程序实现

**文件:** `src/main.rs`

```rust
use rust_rsshub::router;
use rust_rsshub::plugins::PluginRegistry;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};
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
        .unwrap_or_else(|_| "3000".to_string())
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
```

### 9.2 observationalhazard.com 示例配置

**文件:** `configs/observationalhazard.yml`

```yaml
plugin:
  name: "observationalhazard"
  description: "David Kopec's blog RSS feed"
  version: "1.0.0"
  author: "RSSHub"

source:
  type: "html"
  url: "https://www.observationalhazard.com/"
  encoding: "utf-8"
  request:
    method: "GET"
    timeout: 30
    headers:
      Accept: "text/html,application/xhtml+xml,application/xml"
      User-Agent: "Mozilla/5.0 (compatible; RSSHub/1.0)"

parser:
  list:
    selector: "div.post-outer-container"
    link_selector: "a.post-title"
    title_selector: "a.post-title"
    description_selector: "div.post-snippet"
    date_selector: "span.date-header"
    date_format: "%B %d, %Y"

cache:
  enabled: true
  ttl: 3600

feed:
  title: "Observational Hazard"
  description: "David Kopec's blog - Observational Hazard"
  link: "https://www.observationalhazard.com/"
  language: "en"
  format: "rss"
  limit: 20
```

### 9.3 创建 README.md

**文件:** `README.md`

```markdown
# Rust RSSHub

一个基于 Rust 开发的轻量级 RSS 生成器,支持通过配置文件快速为任意网站生成 RSS 订阅。

## 特性

- ✅ **零配置启动**: 无需预加载配置,访问时动态加载
- ✅ **配置驱动**: 通过 YAML 文件定义解析规则,无需编写代码
- ✅ **自动热重载**: 修改配置文件后自动生效,无需重启
- ✅ **多格式支持**: 支持 RSS 2.0 和 Atom 1.0
- ✅ **智能缓存**: 内置内存缓存,提升性能
- ✅ **高性能**: 基于 Rust 和 Tokio 异步运行时
- ✅ **Docker 部署**: 一键部署,易于扩展

## 快速开始

### 使用 Cargo 运行

```bash
# 克隆项目
git clone <repository>
cd rust-rsshub

# 运行
cargo run

# 访问
curl http://localhost:3000/rss/observationalhazard
```

### 使用 Docker

```bash
# 构建镜像
docker build -t rust-rsshub .

# 运行
docker run -p 3000:3000 -v $(pwd)/configs:/app/configs rust-rsshub
```

## 添加新的订阅源

1. 在 `configs/` 目录创建 YAML 配置文件:

```bash
vim configs/mysite.yml
```

2. 配置内容:

```yaml
plugin:
  name: "mysite"
  description: "My Blog RSS Feed"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"
  encoding: "utf-8"

parser:
  list:
    selector: "div.post"
    link_selector: "a.title"
    title_selector: "a.title"
    description_selector: "p.summary"
    date_selector: "span.date"
    date_format: "%Y-%m-%d"

feed:
  title: "My Blog"
  description: "My Blog Feed"
  link: "https://example.com"
  language: "zh"
  format: "rss"
  limit: 20
```

3. **立即访问**,无需重启:

```bash
curl http://localhost:3000/rss/mysite
```

## API 端点

| 端点 | 方法 | 描述 |
|------|------|------|
| `/` | GET | 欢迎页面 |
| `/health` | GET | 健康检查 |
| `/plugins` | GET | 列出所有插件 |
| `/rss/:name` | GET | 获取 RSS 订阅 |
| `/rss/:name?format=atom` | GET | 获取 Atom 订阅 |

## 配置文件格式

详细的配置文件格式请参考 [docs/PLUGIN_GUIDE.md](docs/PLUGIN_GUIDE.md)

## 许可证

MIT License
```

---

## 第十阶段: Docker 部署配置

### 10.1 Dockerfile

**文件:** `Dockerfile`

```dockerfile
# 构建阶段
FROM rust:1.83-alpine AS builder

# 安装依赖
RUN apk add --no-cache musl-dev pkgconfig

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟目录结构
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    touch src/lib.rs

# 构建依赖(利用缓存)
RUN cargo build --release && \
    rm -rf src

# 复制源代码
COPY src ./src

# 构建实际项目
RUN touch src/main.rs && \
    cargo build --release

# 运行阶段
FROM alpine:latest

# 安装运行时依赖
RUN apk add --no-cache ca-certificates

# 创建用户
RUN addgroup -S rsshub && \
    adduser -S rsshub -G rsshub

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/rust-rsshub /app/rust-rsshub

# 创建配置目录
RUN mkdir -p /app/configs && \
    chown -R rsshub:rsshub /app

# 切换用户
USER rsshub

# 暴露端口
EXPOSE 3000

# 设置环境变量
ENV CONFIGS_DIR=/app/configs
ENV PORT=3000

# 启动应用
CMD ["/app/rust-rsshub"]
```

### 10.2 docker-compose.yml

**文件:** `docker-compose.yml`

```yaml
version: '3.8'

services:
  rsshub:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: rust-rsshub
    ports:
      - "3000:3000"
    volumes:
      # 挂载配置目录
      - ./configs:/app/configs:ro
    environment:
      - RUST_LOG=info
      - PORT=3000
      - CONFIGS_DIR=/app/configs
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "-q", "--spider", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

### 10.3 .dockerignore

**文件:** `.dockerignore`

```
target/
Dockerfile
.dockerignore
docker-compose.yml
.git
.gitignore
*.md
docs/
tests/
.env
```

---

## 实施顺序建议

### 第 1 周: 基础框架
- Day 1-2: 第一阶段(项目初始化)
- Day 3-4: 第二阶段(核心数据结构)
- Day 5-7: 第三阶段(配置系统)

### 第 2 周: 核心功能
- Day 1-3: 第四阶段(HTTP 客户端)
- Day 4-6: 第五阶段(解析器系统)
- Day 7: 第六阶段(RSS 生成器)

### 第 3 周: Web 服务
- Day 1-3: 第七阶段(Web 服务器)
- Day 4-5: 第八阶段(插件系统)
- Day 6-7: 第九阶段(主程序和测试)

### 第 4 周: 部署和文档
- Day 1-2: 第十阶段(Docker 配置)
- Day 3-5: 文档编写
- Day 6-7: 测试和优化

---

## 验证检查清单

每个阶段完成后,使用以下清单验证:

### 第一阶段
- [ ] Cargo 项目成功编译
- [ ] 目录结构完整
- [ ] 所有依赖正确配置

### 第二阶段
- [ ] 所有类型定义编译通过
- [ ] 错误类型完整
- [ ] 单元测试通过

### 第三阶段
- [ ] 配置解析器工作正常
- [ ] 能正确读取和验证 YAML 文件
- [ ] 文件存在检查正常

### 第四阶段
- [ ] HTTP 客户端能成功请求网页
- [ ] 编码检测正常
- [ ] 缓存读写正常

### 第五阶段
- [ ] HTML 解析器能提取内容
- [ ] CSS 选择器工作正常
- [ ] 日期解析正确

### 第六阶段
- [ ] RSS 2.0 生成正确
- [ ] Atom 1.0 生成正确
- [ ] 生成的 XML 可被验证

### 第七阶段
- [ ] Web 服务器成功启动
- [ ] 所有路由响应正常
- [ ] 错误处理正确

### 第八阶段
- [ ] 插件动态加载正常
- [ ] 缓存自动刷新正常
- [ ] 并发安全

### 第九阶段
- [ ] observationalhazard.com 示例工作
- [ ] 完整流程测试通过

### 第十阶段
- [ ] Docker 镜像构建成功
- [ ] docker-compose 启动正常
- [ ] 健康检查通过

---

## 故障排查

### 常见问题

1. **编译错误**: 检查 Rust 版本 >= 1.83
2. **CSS 选择器不工作**: 使用浏览器开发工具验证选择器
3. **日期解析失败**: 检查 date_format 格式字符串
4. **编码问题**: 确认 encoding 字段正确
5. **Docker 容器无法启动**: 检查端口占用和权限

---

## 总结

本实施计划提供了完整的技术实现路径,包括:
- ✅ 10 个开发阶段的详细步骤
- ✅ 完整的代码实现(函数签名、数据结构)
- ✅ 依赖清单和配置文件
- ✅ Docker 部署方案
- ✅ 验证检查清单

按照此计划实施,可以在 4 周内完成整个系统的开发和部署。
