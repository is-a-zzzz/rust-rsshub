use rust_rsshub::config::ConfigParser;
use std::path::PathBuf;

#[test]
fn test_config_parser_load_plugin() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    // 测试插件存在性检查
    assert!(parser.plugin_exists("test-example"));

    // 测试配置加载
    let config = parser.load_plugin("test-example").unwrap();

    // 验证插件元数据
    assert_eq!(config.plugin.name, "test-example");
    assert_eq!(config.plugin.description, "Test example plugin for parsing");
    assert_eq!(config.plugin.version, "1.0.0");
    assert_eq!(config.plugin.author, Some("Test Author".to_string()));

    // 验证缓存配置
    assert!(config.cache.enabled);
    assert_eq!(config.cache.ttl, 3600);

    // 验证 Feed 配置
    assert_eq!(config.feed.title, "Example Feed");
    assert_eq!(config.feed.description, "Example RSS Feed");
    assert_eq!(config.feed.link, "https://example.com");
    assert_eq!(config.feed.language, "en");
    assert_eq!(config.feed.format, "rss");
    assert_eq!(config.feed.limit, 20);
}

#[test]
fn test_config_parser_list_plugins() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    let plugins = parser.list_plugins().unwrap();

    // 应该至少包含 test-example
    assert!(plugins.contains(&"test-example".to_string()));
}

#[test]
fn test_config_parser_plugin_not_found() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    // 测试不存在的插件
    assert!(!parser.plugin_exists("non-existent-plugin"));

    // 尝试加载不存在的插件应该返回错误
    let result = parser.load_plugin("non-existent-plugin");
    assert!(result.is_err());
}
