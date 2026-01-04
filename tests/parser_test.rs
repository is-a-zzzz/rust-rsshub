use rust_rsshub::parser::HtmlParser;
use rust_rsshub::ParserConfig;
use rust_rsshub::ListParserConfig;

#[test]
fn test_parse_simple_html() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">First Article</h2>
                <a class="link" href="/article/1">Read more</a>
                <p class="desc">Description 1</p>
                <time class="date">2024-01-15</time>
                <span class="author">Author 1</span>
            </div>
            <div class="post">
                <h2 class="title">Second Article</h2>
                <a class="link" href="/article/2">Read more</a>
                <p class="desc">Description 2</p>
                <time class="date">2024-01-16</time>
                <span class="author">Author 2</span>
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
            description_selector: Some(".desc".to_string()),
            date_selector: Some(".date".to_string()),
            date_format: Some("%Y-%m-%d".to_string()),
            author_selector: Some(".author".to_string()),
            category_selector: None,
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 2);

    // 验证第一篇文章
    assert_eq!(articles[0].title, "First Article");
    assert_eq!(articles[0].link, "https://example.com/article/1");
    assert_eq!(articles[0].description, Some("Description 1".to_string()));
    assert_eq!(articles[0].author, Some("Author 1".to_string()));
    assert!(articles[0].pub_date.is_some());

    // 验证第二篇文章
    assert_eq!(articles[1].title, "Second Article");
    assert_eq!(articles[1].link, "https://example.com/article/2");
    assert_eq!(articles[1].description, Some("Description 2".to_string()));
    assert_eq!(articles[1].author, Some("Author 2".to_string()));
}

#[test]
fn test_parse_with_limit() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article 1</h2>
                <a class="link" href="/1">Link</a>
            </div>
            <div class="post">
                <h2 class="title">Article 2</h2>
                <a class="link" href="/2">Link</a>
            </div>
            <div class="post">
                <h2 class="title">Article 3</h2>
                <a class="link" href="/3">Link</a>
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

    // 默认限制是20，但这里只有3篇文章
    assert_eq!(articles.len(), 3);
}

#[test]
fn test_parse_skip_empty_title() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title"></h2>
                <a class="link" href="/article/1">Link</a>
            </div>
            <div class="post">
                <h2 class="title">Valid Article</h2>
                <a class="link" href="/article/2">Link</a>
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

    // 应该跳过标题为空的文章
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Valid Article");
}

#[test]
fn test_parse_with_absolute_url() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="https://other.com/article">Link</a>
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
    // 绝对 URL 应该保持不变
    assert_eq!(articles[0].link, "https://other.com/article");
}

#[test]
fn test_parse_with_relative_url() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="/post/123">Link</a>
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
    // 相对 URL 应该被转换为绝对 URL
    assert_eq!(articles[0].link, "https://example.com/post/123");
}

#[test]
fn test_parse_with_nested_selector() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <div class="content">
                    <h2 class="title">Nested Title</h2>
                    <div class="meta">
                        <a class="link" href="/article">Link</a>
                    </div>
                </div>
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
    assert_eq!(articles[0].title, "Nested Title");
    assert_eq!(articles[0].link, "https://example.com/article");
}

#[test]
fn test_parse_with_category() {
    let html = r#"
    <html>
        <body>
            <div class="post">
                <h2 class="title">Article</h2>
                <a class="link" href="/article">Link</a>
                <span class="category">Technology</span>
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
            category_selector: Some(".category".to_string()),
        },
        content: None,
    };

    let articles = HtmlParser::parse(html, &config, "https://example.com", 20).unwrap();

    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].category, Some("Technology".to_string()));
}

#[test]
fn test_parse_with_missing_optional_fields() {
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
    assert_eq!(articles[0].title, "Article");
    assert!(articles[0].description.is_none());
    assert!(articles[0].pub_date.is_none());
    assert!(articles[0].author.is_none());
    assert!(articles[0].category.is_none());
}
