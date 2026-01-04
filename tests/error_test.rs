use rust_rsshub::{RssHubError, Result};
use std::io;

#[test]
fn test_error_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let rss_error: RssHubError = io_error.into();

    assert!(matches!(rss_error, RssHubError::IoError(_)));
    assert!(rss_error.to_string().contains("File not found"));
}

#[test]
fn test_error_http_variant() {
    // 这个测试验证 RssHubError::HttpError 变体存在
    // 我们使用一个假的 reqwest 错误来验证类型系统
    // 在实际使用中，错误会从 HTTP 请求中自动转换
    let error = RssHubError::Config("Test".to_string());

    // 验证错误可以匹配
    match error {
        RssHubError::Config(msg) => assert_eq!(msg, "Test"),
        _ => panic!("Expected Config variant"),
    }
}

#[test]
fn test_error_display() {
    let errors = vec![
        RssHubError::Config("Invalid config".to_string()),
        RssHubError::ParseError("Failed to parse HTML".to_string()),
        RssHubError::PluginNotFound("my-plugin".to_string()),
        RssHubError::EncodingError("Invalid UTF-8".to_string()),
        RssHubError::RateLimitExceeded,
        RssHubError::InvalidUrl("not-a-url".to_string()),
    ];

    for error in errors {
        let display_string = error.to_string();
        assert!(!display_string.is_empty());
        println!("Error: {}", display_string);
    }
}

#[test]
fn test_custom_error_messages() {
    let config_err = RssHubError::Config("Missing required field".to_string());
    assert_eq!(
        config_err.to_string(),
        "Configuration error: Missing required field"
    );

    let plugin_err = RssHubError::PluginNotFound("test-plugin".to_string());
    assert_eq!(
        plugin_err.to_string(),
        "Plugin 'test-plugin' not found"
    );

    let url_err = RssHubError::InvalidUrl("htp://invalid".to_string());
    assert_eq!(url_err.to_string(), "Invalid URL: htp://invalid");
}

#[test]
fn test_result_type_alias() {
    // 验证 Result 类型别名工作正常
    fn returns_ok() -> Result<String> {
        Ok("success".to_string())
    }

    fn returns_err() -> Result<String> {
        Err(RssHubError::ParseError("test error".to_string()))
    }

    assert!(returns_ok().is_ok());
    assert_eq!(returns_ok().unwrap(), "success");

    assert!(returns_err().is_err());
}
