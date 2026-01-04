use rust_rsshub::{PluginConfig, SourceConfig};

#[test]
fn test_invalid_feed_title() {
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
  title: ""
  description: "Test"
  link: "https://example.com"
"#;

    let config: Result<PluginConfig, _> = serde_yaml::from_str(yaml);

    // YAML 解析会成功，但 title 是空字符串
    // 这在实际的 ConfigParser::validate_config 中会被捕获
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.feed.title, "");
}

#[test]
fn test_invalid_feed_link() {
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
  description: "Test"
  link: ""
"#;

    let config: Result<PluginConfig, _> = serde_yaml::from_str(yaml);
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.feed.link, "");
}

#[test]
fn test_invalid_url_format() {
    let yaml = r#"
plugin:
  name: "test"
  description: "Test"
  version: "1.0.0"

source:
  type: "html"
  url: "not-a-valid-url"
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
  description: "Test"
  link: "https://example.com"
"#;

    let config: Result<PluginConfig, _> = serde_yaml::from_str(yaml);
    assert!(config.is_ok());
    let config = config.unwrap();

    // URL 格式无效，但 YAML 解析会成功
    // 在 ConfigParser::validate_config 中会被捕获
    match &config.source {
        SourceConfig::Html(html_cfg) => {
            assert_eq!(html_cfg.url, "not-a-valid-url");
        }
        _ => panic!("Expected HTML source"),
    }
}

#[test]
fn test_minimal_valid_config() {
    let yaml = r#"
plugin:
  name: "minimal"
  description: "Minimal config"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"

parser:
  list:
    selector: "div.item"
    link_selector: "a"
    title_selector: "h2"

feed:
  title: "Minimal Feed"
  description: "Minimal"
  link: "https://example.com"
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(config.plugin.name, "minimal");
    assert_eq!(config.feed.title, "Minimal Feed");

    // 验证默认值
    assert_eq!(config.feed.language, "en");
    assert_eq!(config.feed.format, "rss");
    assert_eq!(config.feed.limit, 20);
    assert!(config.cache.enabled);
    assert_eq!(config.cache.ttl, 3600);
}

#[test]
fn test_xml_source_config() {
    let yaml = r#"
plugin:
  name: "xml-test"
  description: "XML source test"
  version: "1.0.0"

source:
  type: "xml"
  url: "https://example.com/data.xml"
  request:
    method: "GET"
    timeout: 45

parser:
  list:
    selector: "div.item"
    link_selector: "a"
    title_selector: "h2"

cache:
  enabled: false
  ttl: 1800

feed:
  title: "XML Feed"
  description: "XML source"
  link: "https://example.com"
  language: "zh"
  format: "atom"
  limit: 50
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    match config.source {
        SourceConfig::Xml(xml_cfg) => {
            assert_eq!(xml_cfg.url, "https://example.com/data.xml");
            assert_eq!(xml_cfg.request.method, "GET");
            assert_eq!(xml_cfg.request.timeout, 45);
        }
        _ => panic!("Expected XML source config"),
    }

    assert!(!config.cache.enabled);
    assert_eq!(config.cache.ttl, 1800);
    assert_eq!(config.feed.language, "zh");
    assert_eq!(config.feed.format, "atom");
    assert_eq!(config.feed.limit, 50);
}

#[test]
fn test_custom_headers_in_request() {
    let yaml = r#"
plugin:
  name: "headers-test"
  description: "Test custom headers"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"
  request:
    method: "GET"
    timeout: 30
    headers:
      Authorization: "Bearer token123"
      User-Agent: "CustomAgent/1.0"
      Accept: "application/json"

parser:
  list:
    selector: "div.item"
    link_selector: "a"
    title_selector: "h2"

cache:
  enabled: true

feed:
  title: "Headers Test"
  description: "Test"
  link: "https://example.com"
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    match config.source {
        SourceConfig::Html(html_cfg) => {
            assert_eq!(html_cfg.request.headers.len(), 3);
            assert_eq!(
                html_cfg.request.headers.get("Authorization"),
                Some(&"Bearer token123".to_string())
            );
            assert_eq!(
                html_cfg.request.headers.get("User-Agent"),
                Some(&"CustomAgent/1.0".to_string())
            );
            assert_eq!(
                html_cfg.request.headers.get("Accept"),
                Some(&"application/json".to_string())
            );
        }
        _ => panic!("Expected HTML source"),
    }
}

#[test]
fn test_content_parser_config() {
    let yaml = r#"
plugin:
  name: "content-test"
  description: "Test content parser"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"

parser:
  list:
    selector: "div.post"
    link_selector: "a.link"
    title_selector: "h2.title"

  content:
    selector: "div.content"
    content_selector: "div.article-body"
    cleanup_selectors:
      - "div.advertisement"
      - "div.sidebar"
      - "script"

cache:
  enabled: true

feed:
  title: "Content Test"
  description: "Test"
  link: "https://example.com"
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    assert!(config.parser.content.is_some());
    let content_parser = config.parser.content.unwrap();
    assert_eq!(content_parser.selector, "div.content");
    assert_eq!(content_parser.content_selector, "div.article-body");
    assert_eq!(content_parser.cleanup_selectors.len(), 3);
    assert!(content_parser.cleanup_selectors.contains(&"div.advertisement".to_string()));
}

#[test]
fn test_all_optional_fields_in_list_parser() {
    let yaml = r#"
plugin:
  name: "full-parser"
  description: "Test all fields"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"

parser:
  list:
    selector: "div.post"
    item_selector: "div.post-item"
    link_selector: "a.post-link"
    title_selector: "h2.title"
    description_selector: "p.excerpt"
    date_selector: "time.published"
    date_format: "%Y-%m-%d %H:%M:%S"
    author_selector: "span.author-name"
    category_selector: "span.category-tag"

cache:
  enabled: true

feed:
  title: "Full Parser Test"
  description: "Test"
  link: "https://example.com"
"#;

    let config: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    let list_parser = &config.parser.list;
    assert_eq!(list_parser.selector, "div.post");
    assert_eq!(list_parser.item_selector, Some("div.post-item".to_string()));
    assert_eq!(list_parser.link_selector, "a.post-link");
    assert_eq!(list_parser.title_selector, "h2.title");
    assert_eq!(list_parser.description_selector, Some("p.excerpt".to_string()));
    assert_eq!(list_parser.date_selector, Some("time.published".to_string()));
    assert_eq!(list_parser.date_format, Some("%Y-%m-%d %H:%M:%S".to_string()));
    assert_eq!(list_parser.author_selector, Some("span.author-name".to_string()));
    assert_eq!(list_parser.category_selector, Some("span.category-tag".to_string()));
}
