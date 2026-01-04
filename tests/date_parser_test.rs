use rust_rsshub::parser::HtmlParser;
use rust_rsshub::{ParserConfig, ListParserConfig};

#[test]
fn test_parse_with_date_iso() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="/article">Link</a>
                <time class="date">2024-01-15</time>
            </div>
        </body>
    </html>
    "#;

    let config = ParserConfig {
        list: ListParserConfig {
            selector: ".post".to_string(),
            item_selector: None,
            link_selector: ".link".to_string(),
            title_selector: ".title".to_string(),
            description_selector: None,
            date_selector: Some(".date".to_string()),
            date_format: Some("%Y-%m-%d".to_string()),
            author_selector: None,
            category_selector: None,
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 1);
    assert!(articles[0].pub_date.is_some());
}

#[test]
fn test_parse_with_date_rfc3339() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="/article">Link</a>
                <time class="date">2024-01-15T10:30:00Z</time>
            </div>
        </body>
    </html>
    "#;

    let config = ParserConfig {
        list: ListParserConfig {
            selector: ".post".to_string(),
            item_selector: None,
            link_selector: ".link".to_string(),
            title_selector: ".title".to_string(),
            description_selector: None,
            date_selector: Some(".date".to_string()),
            date_format: None,
            author_selector: None,
            category_selector: None,
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 1);
    assert!(articles[0].pub_date.is_some());
}

#[test]
fn test_parse_with_date_month_name() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="/article">Link</a>
                <time class="date">January 15, 2024</time>
            </div>
        </body>
    </html>
    "#;

    let config = ParserConfig {
        list: ListParserConfig {
            selector: ".post".to_string(),
            item_selector: None,
            link_selector: ".link".to_string(),
            title_selector: ".title".to_string(),
            description_selector: None,
            date_selector: Some(".date".to_string()),
            date_format: None,
            author_selector: None,
            category_selector: None,
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 1);
    assert!(articles[0].pub_date.is_some());
}

#[test]
fn test_parse_with_multiple_dates() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article 1</h2>
                <a class="link" href="/1">Link</a>
                <time class="date">2024-01-15</time>
            </div>
            <div class="post">
                <h2 class="title">Article 2</h2>
                <a class="link" href="/2">Link</a>
                <time class="date">January 16, 2024</time>
            </div>
            <div class="post">
                <h2 class="title">Article 3</h2>
                <a class="link" href="/3">Link</a>
                <time class="date">2024-01-17T10:30:00Z</time>
            </div>
        </body>
    </html>
    "#;

    let config = ParserConfig {
        list: ListParserConfig {
            selector: ".post".to_string(),
            item_selector: None,
            link_selector: ".link".to_string(),
            title_selector: ".title".to_string(),
            description_selector: None,
            date_selector: Some(".date".to_string()),
            date_format: None,
            author_selector: None,
            category_selector: None,
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 3);
    // 所有文章都应该有日期
    for article in &articles {
        assert!(article.pub_date.is_some());
    }
}

#[test]
fn test_parse_with_invalid_date() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="/article">Link</a>
                <time class="date">Invalid Date</time>
            </div>
        </body>
    </html>
    "#;

    let config = ParserConfig {
        list: ListParserConfig {
            selector: ".post".to_string(),
            item_selector: None,
            link_selector: ".link".to_string(),
            title_selector: ".title".to_string(),
            description_selector: None,
            date_selector: Some(".date".to_string()),
            date_format: None,
            author_selector: None,
            category_selector: None,
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 1);
    // 无效日期应该返回 None
    assert!(articles[0].pub_date.is_none());
}

#[test]
fn test_parse_with_empty_date() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="/article">Link</a>
                <time class="date"></time>
            </div>
        </body>
    </html>
    "#;

    let config = ParserConfig {
        list: ListParserConfig {
            selector: ".post".to_string(),
            item_selector: None,
            link_selector: ".link".to_string(),
            title_selector: ".title".to_string(),
            description_selector: None,
            date_selector: Some(".date".to_string()),
            date_format: None,
            author_selector: None,
            category_selector: None,
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 1);
    // 空日期应该返回 None
    assert!(articles[0].pub_date.is_none());
}

#[test]
fn test_parse_without_date_field() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="/article">Link</a>
            </div>
        </body>
    </html>
    "#;

    let config = ParserConfig {
        list: ListParserConfig {
            selector: ".post".to_string(),
            item_selector: None,
            link_selector: ".link".to_string(),
            title_selector: ".title".to_string(),
            description_selector: None,
            date_selector: None,
            date_format: None,
            author_selector: None,
            category_selector: None,
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 1);
    // 没有日期字段
    assert!(articles[0].pub_date.is_none());
}
