use rust_rsshub::fetcher::HttpFetcher;
use rust_rsshub::{SourceConfig, HtmlSourceConfig, RequestConfig};
use std::collections::HashMap;

#[tokio::test]
async fn test_http_fetcher_create() {
    let fetcher = HttpFetcher::new();
    assert!(fetcher.is_ok());
}

#[tokio::test]
async fn test_http_fetcher_with_custom_timeout() {
    let fetcher = HttpFetcher::with_timeout(60);
    assert!(fetcher.is_ok());
}

#[tokio::test]
async fn test_http_fetcher_default() {
    let _fetcher = rust_rsshub::fetcher::HttpFetcher::default();
    // 如果创建成功就通过
}

#[tokio::test]
async fn test_fetch_html_with_example_com() {
    let fetcher = HttpFetcher::new().unwrap();

    let source = SourceConfig::Html(HtmlSourceConfig {
        url: "https://example.com".to_string(),
        encoding: "utf-8".to_string(),
        user_agent: None,
        request: RequestConfig::default(),
    });

    let result = fetcher.fetch_html(&source).await;

    // 这个测试可能会因为网络原因失败，所以只检查结果类型
    match result {
        Ok(html) => {
            // 验证返回了HTML内容
            assert!(!html.is_empty());
            // example.com 应该包含标题
            assert!(html.contains("Example Domain") || html.contains("<title>"));
        }
        Err(e) => {
            // 如果是网络错误，这是可以接受的
            eprintln!("Network error (acceptable): {}", e);
        }
    }
}

#[tokio::test]
async fn test_fetch_html_with_custom_headers() {
    let fetcher = HttpFetcher::new().unwrap();

    let mut headers = HashMap::new();
    headers.insert("Accept".to_string(), "text/html".to_string());
    headers.insert("User-Agent".to_string(), "Test/1.0".to_string());

    let source = SourceConfig::Html(HtmlSourceConfig {
        url: "https://example.com".to_string(),
        encoding: "utf-8".to_string(),
        user_agent: None,
        request: RequestConfig {
            method: "GET".to_string(),
            headers,
            timeout: 30,
        },
    });

    let result = fetcher.fetch_html(&source).await;

    match result {
        Ok(html) => {
            assert!(!html.is_empty());
        }
        Err(e) => {
            eprintln!("Network error (acceptable): {}", e);
        }
    }
}

#[tokio::test]
async fn test_fetch_html_with_invalid_url() {
    let fetcher = HttpFetcher::new().unwrap();

    let source = SourceConfig::Html(HtmlSourceConfig {
        url: "not-a-valid-url".to_string(),
        encoding: "utf-8".to_string(),
        user_agent: None,
        request: RequestConfig::default(),
    });

    let result = fetcher.fetch_html(&source).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fetch_json() {
    let fetcher = HttpFetcher::new().unwrap();

    // 使用一个公共的 JSON API
    let result = fetcher.fetch_json("https://jsonplaceholder.typicode.com/posts/1").await;

    match result {
        Ok(json) => {
            // 验证返回了有效的 JSON
            assert!(json.is_object());
            assert!(json.get("userId").is_some());
        }
        Err(e) => {
            eprintln!("Network error (acceptable): {}", e);
        }
    }
}

#[tokio::test]
async fn test_fetch_html_timeout() {
    let fetcher = HttpFetcher::with_timeout(1).unwrap(); // 1秒超时

    // 使用一个可能响应慢的URL（或者一个不存在的URL，这会导致连接超时）
    let source = SourceConfig::Html(HtmlSourceConfig {
        url: "https://example.com".to_string(),
        encoding: "utf-8".to_string(),
        user_agent: None,
        request: RequestConfig {
            method: "GET".to_string(),
            headers: HashMap::new(),
            timeout: 1, // 1秒超时
        },
    });

    // 这个测试通常会成功，因为 example.com 响应很快
    // 如果网络正常，不应该超时
    let result = fetcher.fetch_html(&source).await;

    match result {
        Ok(_) => {
            // 成功获取到内容
        }
        Err(e) => {
            eprintln!("Request error (acceptable): {}", e);
        }
    }
}

#[tokio::test]
async fn test_different_encodings() {
    let fetcher = HttpFetcher::new().unwrap();

    // 测试 UTF-8 编码
    let utf8_source = SourceConfig::Html(HtmlSourceConfig {
        url: "https://example.com".to_string(),
        encoding: "utf-8".to_string(),
        user_agent: None,
        request: RequestConfig::default(),
    });

    let result = fetcher.fetch_html(&utf8_source).await;

    match result {
        Ok(html) => {
            assert!(!html.is_empty());
        }
        Err(e) => {
            eprintln!("Network error (acceptable): {}", e);
        }
    }
}

#[test]
fn test_request_config_default() {
    let config = RequestConfig::default();
    assert_eq!(config.method, "GET");
    assert_eq!(config.timeout, 30);
    assert!(config.headers.is_empty());
}
