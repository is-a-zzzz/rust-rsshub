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
    pub category: Option<String>,
    #[serde(default)]
    pub guid: Option<String>,
}

/// RSS Feed 数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssFeed {
    pub title: String,
    pub description: String,
    pub link: String,
    pub language: String,
    pub articles: Vec<Article>,
}
