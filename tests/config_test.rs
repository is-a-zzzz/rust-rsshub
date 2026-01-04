use rust_rsshub::{PluginConfig, SourceConfig, CacheConfig};

#[test]
fn test_cache_config_default() {
    let cache = CacheConfig::default();
    assert!(cache.enabled);
    assert_eq!(cache.ttl, 3600);
}

#[test]
fn test_yaml_deserialization() {
    let yaml = r#"
plugin:
  name: "test"
  description: "Test plugin"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"
  encoding: "utf-8"
  request:
    method: "GET"
    timeout: 30

parser:
  list:
    selector: "div.post"
    link_selector: "a.title"
    title_selector: "a.title"

cache:
  enabled: true
  ttl: 3600

feed:
  title: "Test Feed"
  description: "Test Description"
  link: "https://example.com"
  language: "en"
  format: "rss"
  limit: 20
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(config.plugin.name, "test");
    assert_eq!(config.plugin.version, "1.0.0");
    assert!(config.cache.enabled);
    assert_eq!(config.cache.ttl, 3600);
    assert_eq!(config.feed.limit, 20);
}

#[test]
fn test_source_config_html() {
    let yaml = r#"
plugin:
  name: "test"
  description: "Test"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"
  encoding: "utf-8"
  request:
    method: "GET"
    timeout: 30

parser:
  list:
    selector: "div.post"
    link_selector: "a"
    title_selector: "a"

cache:
  enabled: true
  ttl: 3600

feed:
  title: "Test"
  description: "Test"
  link: "https://example.com"
  language: "en"
  format: "rss"
  limit: 20
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    match config.source {
        SourceConfig::Html(html_cfg) => {
            assert_eq!(html_cfg.url, "https://example.com");
            assert_eq!(html_cfg.encoding, "utf-8");
            assert_eq!(html_cfg.request.method, "GET");
            assert_eq!(html_cfg.request.timeout, 30);
        }
        _ => panic!("Expected Html source config"),
    }
}

#[test]
fn test_source_config_json() {
    let yaml = r#"
plugin:
  name: "test"
  description: "Test"
  version: "1.0.0"

source:
  type: "json"
  url: "https://api.example.com/data"
  request:
    method: "POST"
    timeout: 60

parser:
  list:
    selector: "div.post"
    link_selector: "a"
    title_selector: "a"

cache:
  enabled: false
  ttl: 7200

feed:
  title: "Test"
  description: "Test"
  link: "https://example.com"
  language: "en"
  format: "rss"
  limit: 10
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    match config.source {
        SourceConfig::Json(json_cfg) => {
            assert_eq!(json_cfg.url, "https://api.example.com/data");
            assert_eq!(json_cfg.request.method, "POST");
            assert_eq!(json_cfg.request.timeout, 60);
        }
        _ => panic!("Expected Json source config"),
    }

    assert!(!config.cache.enabled);
    assert_eq!(config.cache.ttl, 7200);
    assert_eq!(config.feed.limit, 10);
}

#[test]
fn test_feed_config_defaults() {
    let yaml = r#"
plugin:
  name: "test"
  description: "Test"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"
  request:
    method: "GET"
    timeout: 30

parser:
  list:
    selector: "div.post"
    link_selector: "a"
    title_selector: "a"

cache:
  enabled: true

feed:
  title: "Test Feed"
  description: "Test Description"
  link: "https://example.com"
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    // 验证默认值
    assert_eq!(config.feed.language, "en");
    assert_eq!(config.feed.format, "rss");
    assert_eq!(config.feed.limit, 20);
}
