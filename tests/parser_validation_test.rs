use rust_rsshub::config::ConfigParser;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_validate_config_with_invalid_title() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    // 创建一个临时配置文件，标题为空
    let yaml = r#"
plugin:
  name: "invalid-title"
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

    let test_config_path = configs_dir.join("invalid-title.yml");
    fs::write(&test_config_path, yaml).unwrap();

    // 尝试加载配置 - 应该失败因为标题为空
    let result = parser.load_plugin("invalid-title");

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Feed title cannot be empty"));
    }

    // 清理
    let _ = fs::remove_file(&test_config_path);
}

#[test]
fn test_validate_config_with_invalid_link() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    let yaml = r#"
plugin:
  name: "invalid-link"
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

    let test_config_path = configs_dir.join("invalid-link.yml");
    fs::write(&test_config_path, yaml).unwrap();

    let result = parser.load_plugin("invalid-link");

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Feed link cannot be empty"));
    }

    // 清理
    let _ = fs::remove_file(&test_config_path);
}

#[test]
fn test_validate_config_with_invalid_url() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    let yaml = r#"
plugin:
  name: "invalid-url"
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

    let test_config_path = configs_dir.join("invalid-url.yml");
    fs::write(&test_config_path, yaml).unwrap();

    let result = parser.load_plugin("invalid-url");

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Invalid URL"));
    }

    // 清理
    let _ = fs::remove_file(&test_config_path);
}

#[test]
fn test_validate_config_with_missing_file() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    let result = parser.load_plugin("non-existent-plugin");
    assert!(result.is_err());
}

#[test]
fn test_list_plugins_with_various_files() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    // 创建一些测试配置文件
    let test_files = vec![
        ("plugin1.yml", "plugin1"),
        ("plugin2.yml", "plugin2"),
        ("plugin3.yaml", "plugin3"),
        (".hidden.yml", "hidden"),  // 应该被忽略
    ];

    for (filename, plugin_name) in &test_files {
        let yaml = format!(r#"
plugin:
  name: "{}"
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
  title: "Test"
  description: "Test"
  link: "https://example.com"
"#, plugin_name);

        let path = configs_dir.join(filename);
        fs::write(&path, yaml).unwrap();
    }

    let plugins = parser.list_plugins().unwrap();

    // 应该包含 plugin1, plugin2, plugin3，但不包含 .hidden
    assert!(plugins.contains(&"plugin1".to_string()));
    assert!(plugins.contains(&"plugin2".to_string()));
    assert!(plugins.contains(&"plugin3".to_string()));
    assert!(!plugins.contains(&"hidden".to_string()));

    // 清理
    for (filename, _) in &test_files {
        let path = configs_dir.join(filename);
        let _ = fs::remove_file(&path);
    }
}

#[test]
fn test_get_plugin_mtime() {
    let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");
    let parser = ConfigParser::new(configs_dir.to_string_lossy().to_string());

    // 使用现有的 test-example.yml
    let mtime = parser.get_plugin_mtime("test-example");

    assert!(mtime.is_ok());
    let time = mtime.unwrap();
    // 验证时间是一个合理的时间戳（在过去）
    assert!(time.duration_since(std::time::UNIX_EPOCH).is_ok());
}
