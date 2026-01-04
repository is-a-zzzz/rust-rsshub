use crate::config::types::{Article, RssFeed};
use rss::{ChannelBuilder, Item, GuidBuilder};

pub struct RssGenerator;

impl RssGenerator {
    pub fn generate(feed: &RssFeed) -> String {
        let items: Vec<Item> = feed
            .articles
            .iter()
            .map(|article| Self::article_to_item(article))
            .collect();

        let channel = ChannelBuilder::default()
            .title(&feed.title)
            .link(&feed.link)
            .description(&feed.description)
            .language(Some(feed.language.clone()))
            .items(items)
            .build();

        channel.to_string()
    }

    fn article_to_item(article: &Article) -> Item {
        let mut item = Item::default();

        item.set_title(article.title.clone());
        item.set_link(article.link.clone());

        // 描述
        if let Some(ref desc) = article.description {
            item.set_description(desc.clone());
        }

        // 内容
        if let Some(ref content) = article.content {
            item.set_content(content.clone());
        }

        // 发布日期
        if let Some(pub_date) = article.pub_date {
            item.set_pub_date(pub_date.to_rfc2822());
        }

        // 作者
        if let Some(ref author) = article.author {
            item.set_author(author.clone());
        }

        // 分类
        if let Some(ref category) = article.category {
            item.set_categories(vec![rss::Category {
                name: category.clone(),
                domain: None,
            }]);
        }

        // GUID
        let guid = GuidBuilder::default()
            .value(article.guid.as_ref().unwrap_or(&article.link).clone())
            .permalink(false)
            .build();
        item.set_guid(guid);

        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_generate_rss() {
        let feed = RssFeed {
            title: "Test Feed".to_string(),
            description: "Test Description".to_string(),
            link: "https://example.com".to_string(),
            language: "en".to_string(),
            articles: vec![],
        };

        let rss = RssGenerator::generate(&feed);

        assert!(rss.contains("<title>Test Feed</title>"));
        assert!(rss.contains("<description>Test Description</description>"));
        assert!(rss.contains("<link>https://example.com</link>"));
    }

    #[test]
    fn test_article_with_all_fields() {
        let article = Article {
            title: "Test Article".to_string(),
            link: "https://example.com/article".to_string(),
            description: Some("Test description".to_string()),
            content: Some("<p>Test content</p>".to_string()),
            pub_date: Some(Utc::now()),
            author: Some("John Doe".to_string()),
            category: Some("Tech".to_string()),
            guid: Some("unique-id-123".to_string()),
        };

        let item = RssGenerator::article_to_item(&article);

        assert_eq!(item.title(), Some("Test Article"));
        assert_eq!(item.link(), Some("https://example.com/article"));
        assert_eq!(item.description(), Some("Test description"));
        assert_eq!(item.author(), Some("John Doe"));
    }

    #[test]
    fn test_article_with_minimal_fields() {
        let article = Article {
            title: "Simple Article".to_string(),
            link: "https://example.com/simple".to_string(),
            description: None,
            content: None,
            pub_date: None,
            author: None,
            category: None,
            guid: None,
        };

        let item = RssGenerator::article_to_item(&article);

        assert_eq!(item.title(), Some("Simple Article"));
        assert_eq!(item.link(), Some("https://example.com/simple"));
        assert!(item.description().is_none());
        assert!(item.author().is_none());
    }
}
