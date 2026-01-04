use crate::config::types::{Article, RssFeed};
use atom_syndication::{Feed, Link, Person, Content};

pub struct AtomGenerator;

impl AtomGenerator {
    pub fn generate(feed: &RssFeed) -> String {
        let mut atom_feed = Feed::default();
        atom_feed.set_title(feed.title.clone());
        atom_feed.set_id(feed.link.clone());
        atom_feed.set_links(vec![Link {
            href: feed.link.clone(),
            rel: "alternate".to_string(),
            mime_type: None,
            title: None,
            hreflang: None,
            length: None,
        }]);

        // 设置更新时间
        if let Some(latest_article) = feed.articles.first() {
            if let Some(pub_date) = latest_article.pub_date {
                atom_feed.set_updated(pub_date);
            }
        }

        // 添加文章
        let entries: Vec<atom_syndication::Entry> = feed
            .articles
            .iter()
            .map(|article| Self::article_to_entry(article))
            .collect();

        atom_feed.set_entries(entries);

        atom_feed.to_string()
    }

    fn article_to_entry(article: &Article) -> atom_syndication::Entry {
        let mut entry = atom_syndication::EntryBuilder::default();
        entry.title(article.title.clone());

        // ID
        entry.id(article.guid.as_ref().unwrap_or(&article.link).clone());

        // 链接
        entry.links(vec![Link {
            href: article.link.clone(),
            rel: "alternate".to_string(),
            mime_type: None,
            title: None,
            hreflang: None,
            length: None,
        }]);

        // 内容
        if let Some(ref content) = article.content {
            let mut content_obj = Content::default();
            content_obj.value = Some(content.clone());
            content_obj.content_type = Some("html".to_string());
            entry.content(content_obj);
        } else if let Some(ref desc) = article.description {
            // 如果没有content，使用description作为content
            let mut content_obj = Content::default();
            content_obj.value = Some(desc.clone());
            content_obj.content_type = Some("text".to_string());
            entry.content(content_obj);
        }

        // 摘要
        if let Some(ref summary) = article.description {
            entry.summary(Some(atom_syndication::Text::plain(summary.clone())));
        }

        // 发布日期和更新时间
        if let Some(pub_date) = article.pub_date {
            // 转换为FixedOffset
            let fixed_date: chrono::DateTime<chrono::FixedOffset> = pub_date.clone().into();
            entry.published(Some(fixed_date));
            entry.updated(fixed_date);
        }

        // 作者
        if let Some(ref author) = article.author {
            entry.authors(vec![Person {
                name: author.clone(),
                email: None,
                uri: None,
            }]);
        }

        // 分类
        if let Some(ref category) = article.category {
            entry.categories(vec![atom_syndication::Category {
                term: category.clone(),
                scheme: None,
                label: None,
            }]);
        }

        entry.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_generate_atom() {
        let feed = RssFeed {
            title: "Test Feed".to_string(),
            description: "Test Description".to_string(),
            link: "https://example.com".to_string(),
            language: "en".to_string(),
            articles: vec![],
        };

        let atom = AtomGenerator::generate(&feed);

        assert!(atom.contains("Test Feed"));
        assert!(atom.contains("https://example.com"));
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

        let entry = AtomGenerator::article_to_entry(&article);

        assert_eq!(entry.title(), "Test Article");
        assert_eq!(entry.id(), "unique-id-123");
        assert!(!entry.links().is_empty());
        assert_eq!(entry.links()[0].href, "https://example.com/article");
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

        let entry = AtomGenerator::article_to_entry(&article);

        assert_eq!(entry.title(), "Simple Article");
        assert_eq!(entry.id(), "https://example.com/simple");
    }
}
