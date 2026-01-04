use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct CacheEntry {
    pub content: String,
    pub created_at: Instant,
}

pub struct MemoryCache {
    entries: RwLock<HashMap<String, CacheEntry>>,
    ttl: Duration,
}

impl MemoryCache {
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    /// 获取缓存内容
    pub async fn get(&self, key: &str) -> Option<String> {
        let entries = self.entries.read().await;
        if let Some(entry) = entries.get(key) {
            if entry.created_at.elapsed() < self.ttl {
                return Some(entry.content.clone());
            }
        }
        None
    }

    /// 设置缓存
    pub async fn set(&self, key: String, content: String) {
        let entry = CacheEntry {
            content,
            created_at: Instant::now(),
        };
        let mut entries = self.entries.write().await;
        entries.insert(key, entry);
    }

    /// 清除过期缓存
    pub async fn cleanup(&self) {
        let mut entries = self.entries.write().await;
        entries.retain(|_, entry| entry.created_at.elapsed() < self.ttl);
    }

    /// 清除指定缓存
    pub async fn invalidate(&self, key: &str) {
        let mut entries = self.entries.write().await;
        entries.remove(key);
    }

    /// 清除所有缓存
    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
    }

    /// 获取缓存条目数量
    pub async fn len(&self) -> usize {
        let entries = self.entries.read().await;
        entries.len()
    }

    /// 检查缓存是否为空
    pub async fn is_empty(&self) -> bool {
        let entries = self.entries.read().await;
        entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_set_get() {
        let cache = MemoryCache::new(3600);

        cache.set("key1".to_string(), "value1".to_string()).await;

        let value = cache.get("key1").await;
        assert!(value.is_some());
        assert_eq!(value.unwrap(), "value1");
    }

    #[tokio::test]
    async fn test_cache_expired() {
        let cache = MemoryCache::new(1); // 1秒 TTL

        cache.set("key1".to_string(), "value1".to_string()).await;

        // 立即获取，应该存在
        let value = cache.get("key1").await;
        assert!(value.is_some());

        // 等待2秒后获取，应该过期
        tokio::time::sleep(Duration::from_secs(2)).await;
        let value = cache.get("key1").await;
        assert!(value.is_none());
    }

    #[tokio::test]
    async fn test_cache_invalidate() {
        let cache = MemoryCache::new(3600);

        cache.set("key1".to_string(), "value1".to_string()).await;
        assert!(cache.get("key1").await.is_some());

        cache.invalidate("key1").await;
        assert!(cache.get("key1").await.is_none());
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = MemoryCache::new(3600);

        cache.set("key1".to_string(), "value1".to_string()).await;
        cache.set("key2".to_string(), "value2".to_string()).await;

        assert_eq!(cache.len().await, 2);

        cache.clear().await;

        assert_eq!(cache.len().await, 0);
        assert!(cache.is_empty().await);
    }

    #[tokio::test]
    async fn test_cache_cleanup() {
        let cache = MemoryCache::new(1); // 1秒 TTL

        cache.set("key1".to_string(), "value1".to_string()).await;
        cache.set("key2".to_string(), "value2".to_string()).await;

        assert_eq!(cache.len().await, 2);

        // 等待2秒
        tokio::time::sleep(Duration::from_secs(2)).await;

        // 清理过期条目
        cache.cleanup().await;

        assert_eq!(cache.len().await, 0);
    }
}
