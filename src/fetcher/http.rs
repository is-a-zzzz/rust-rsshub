use crate::error::{Result, RssHubError};
use crate::config::types::SourceConfig;
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
        let (url, config, encoding) = match source {
            SourceConfig::Html(cfg) => (&cfg.url, &cfg.request, cfg.encoding.clone()),
            SourceConfig::Json(cfg) => (&cfg.url, &cfg.request, "utf-8".to_string()),
            SourceConfig::Xml(cfg) => (&cfg.url, &cfg.request, "utf-8".to_string()),
        };

        let mut request = self.client.request(
            reqwest::Method::from_bytes(config.method.as_bytes())
                .map_err(|_| RssHubError::Config(format!("Invalid HTTP method: {}", config.method)))?,
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

        // 检查响应状态
        let status = response.status();
        if !status.is_success() {
            return Err(RssHubError::Config(format!(
                "HTTP request failed with status: {}",
                status
            )));
        }

        // 获取响应字节
        let bytes = response.bytes().await?;

        // 检测编码并转换为 UTF-8
        let encoding = encoding_rs::Encoding::for_label(encoding.as_bytes())
            .unwrap_or(UTF_8);
        let (html, _, _) = encoding.decode(&bytes);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_fetcher_new() {
        let fetcher = HttpFetcher::new();
        assert!(fetcher.is_ok());
    }

    #[test]
    fn test_http_fetcher_default() {
        let _fetcher = HttpFetcher::default();
        // 如果创建成功，测试就通过
    }
}
