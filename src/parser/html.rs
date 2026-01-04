use crate::error::{Result, RssHubError};
use crate::config::types::{ParserConfig, Article};
use scraper::{Html, Selector, ElementRef};
use chrono::DateTime;

pub struct HtmlParser;

impl HtmlParser {
    pub fn parse(html: &str, config: &ParserConfig, base_url: &str, limit: usize) -> Result<Vec<Article>> {
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
            if articles.len() >= limit {
                break;
            }
        }

        Ok(articles)
    }

    fn parse_article(
        element: &ElementRef,
        config: &crate::config::types::ListParserConfig,
        base_url: &str,
    ) -> Result<Option<Article>> {
        // 提取标题
        let title = Self::extract_text(element, &config.title_selector)?;

        // 如果标题为空，跳过这篇文章
        if title.trim().is_empty() {
            return Ok(None);
        }

        // 提取链接
        let link = match Self::extract_link(element, &config.link_selector, base_url) {
            Ok(link) => link,
            Err(_) => return Ok(None), // 如果链接提取失败，跳过
        };

        // 提取描述
        let description = if let Some(ref desc_selector) = config.description_selector {
            let desc = Self::extract_text(element, desc_selector)?;
            if desc.trim().is_empty() {
                None
            } else {
                Some(desc)
            }
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
            let auth = Self::extract_text(element, author_selector)?;
            if auth.trim().is_empty() {
                None
            } else {
                Some(auth)
            }
        } else {
            None
        };

        // 提取分类
        let category = if let Some(ref category_selector) = config.category_selector {
            let cat = Self::extract_text(element, category_selector)?;
            if cat.trim().is_empty() {
                None
            } else {
                Some(cat)
            }
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
            category,
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
                if href.starts_with("http://") || href.starts_with("https://") {
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

        if text.trim().is_empty() {
            return Ok(None);
        }

        // 尝试多种日期格式
        if let Some(fmt) = format {
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&text, fmt) {
                return Ok(Some(dt.and_utc()));
            }
            if let Ok(dt) = chrono::NaiveDate::parse_from_str(&text, fmt) {
                return Ok(Some(dt.and_hms_opt(0, 0, 0).unwrap().and_utc()));
            }
        }

        // 尝试常见格式
        let formats = [
            "%B %d, %Y",
            "%Y-%m-%d",
            "%Y-%m-%dT%H:%M:%S%z",
            "%Y-%m-%dT%H:%M:%SZ",
            "%Y-%m-%dT%H:%M:%S",
            "%a, %d %b %Y %H:%M:%S %z",
            "%a, %d %b %Y %H:%M:%S GMT",
            "%d %b %Y",
            "%d/%m/%Y",
            "%m/%d/%Y",
        ];

        for fmt in &formats {
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&text, fmt) {
                return Ok(Some(dt.and_utc()));
            }
            if let Ok(dt) = chrono::NaiveDate::parse_from_str(&text, fmt) {
                if let Some(naive_dt) = dt.and_hms_opt(0, 0, 0) {
                    return Ok(Some(naive_dt.and_utc()));
                }
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text() {
        let html = r#"
        <div>
            <span class="title">Test Title</span>
        </div>
        "#;

        let document = Html::parse_document(html);
        let element = document.select(&Selector::parse("div").unwrap()).next().unwrap();

        let text = HtmlParser::extract_text(&element, ".title").unwrap();
        assert_eq!(text, "Test Title");
    }

    #[test]
    fn test_extract_link() {
        let html = r#"
        <div>
            <a href="https://example.com/article">Link</a>
        </div>
        "#;

        let document = Html::parse_document(html);
        let element = document.select(&Selector::parse("div").unwrap()).next().unwrap();

        let link = HtmlParser::extract_link(&element, "a", "https://base.com").unwrap();
        assert_eq!(link, "https://example.com/article");
    }

    #[test]
    fn test_extract_relative_link() {
        let html = r#"
        <div>
            <a href="/article/123">Link</a>
        </div>
        "#;

        let document = Html::parse_document(html);
        let element = document.select(&Selector::parse("div").unwrap()).next().unwrap();

        let link = HtmlParser::extract_link(&element, "a", "https://base.com").unwrap();
        assert_eq!(link, "https://base.com/article/123");
    }
}
