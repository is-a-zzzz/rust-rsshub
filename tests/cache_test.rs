use rust_rsshub::fetcher::MemoryCache;
use std::time::Duration;

#[tokio::test]
async fn test_cache_basic_operations() {
    let cache = MemoryCache::new(3600);

    // 测试缓存为空
    assert!(cache.is_empty().await);
    assert_eq!(cache.len().await, 0);

    // 设置缓存
    cache.set("key1".to_string(), "value1".to_string()).await;
    cache.set("key2".to_string(), "value2".to_string()).await;
    cache.set("key3".to_string(), "value3".to_string()).await;

    // 验证缓存不为空
    assert!(!cache.is_empty().await);
    assert_eq!(cache.len().await, 3);

    // 获取缓存
    let value = cache.get("key1").await;
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "value1");

    let value = cache.get("key2").await;
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "value2");

    // 获取不存在的键
    let value = cache.get("nonexistent").await;
    assert!(value.is_none());
}

#[tokio::test]
async fn test_cache_expiration() {
    let cache = MemoryCache::new(1); // 1秒 TTL

    cache.set("key1".to_string(), "value1".to_string()).await;

    // 立即获取，应该存在
    let value = cache.get("key1").await;
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "value1");

    // 等待1.5秒后获取，应该已过期
    tokio::time::sleep(Duration::from_millis(1500)).await;
    let value = cache.get("key1").await;
    assert!(value.is_none());
}

#[tokio::test]
async fn test_cache_invalidate() {
    let cache = MemoryCache::new(3600);

    cache.set("key1".to_string(), "value1".to_string()).await;
    cache.set("key2".to_string(), "value2".to_string()).await;

    assert_eq!(cache.len().await, 2);

    // 删除 key1
    cache.invalidate("key1").await;

    assert_eq!(cache.len().await, 1);
    assert!(cache.get("key1").await.is_none());
    assert!(cache.get("key2").await.is_some());
}

#[tokio::test]
async fn test_cache_clear() {
    let cache = MemoryCache::new(3600);

    // 添加多个条目
    for i in 1..=10 {
        cache.set(format!("key{}", i), format!("value{}", i)).await;
    }

    assert_eq!(cache.len().await, 10);

    // 清除所有缓存
    cache.clear().await;

    assert_eq!(cache.len().await, 0);
    assert!(cache.is_empty().await);

    // 验证所有键都不存在
    for i in 1..=10 {
        assert!(cache.get(&format!("key{}", i)).await.is_none());
    }
}

#[tokio::test]
async fn test_cache_cleanup() {
    let cache = MemoryCache::new(1); // 1秒 TTL

    // 添加一些条目
    cache.set("expired1".to_string(), "value1".to_string()).await;
    cache.set("expired2".to_string(), "value2".to_string()).await;

    // 等待1.5秒
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // 添加新的未过期条目
    cache.set("active1".to_string(), "value3".to_string()).await;
    cache.set("active2".to_string(), "value4".to_string()).await;

    assert_eq!(cache.len().await, 4); // 包括已过期的

    // 清理过期条目
    cache.cleanup().await;

    // 应该只剩下2个未过期的条目
    assert_eq!(cache.len().await, 2);
    assert!(cache.get("expired1").await.is_none());
    assert!(cache.get("expired2").await.is_none());
    assert!(cache.get("active1").await.is_some());
    assert!(cache.get("active2").await.is_some());
}

#[tokio::test]
async fn test_cache_overwrite() {
    let cache = MemoryCache::new(3600);

    cache.set("key1".to_string(), "value1".to_string()).await;
    assert_eq!(cache.get("key1").await.unwrap(), "value1");

    // 覆盖相同的键
    cache.set("key1".to_string(), "value2".to_string()).await;
    assert_eq!(cache.get("key1").await.unwrap(), "value2");

    // 长度应该还是1
    assert_eq!(cache.len().await, 1);
}

#[tokio::test]
async fn test_cache_large_values() {
    let cache = MemoryCache::new(3600);

    // 创建一个大的字符串值
    let large_value = "x".repeat(100_000);

    cache.set("large_key".to_string(), large_value.clone()).await;

    let retrieved = cache.get("large_key").await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().len(), 100_000);
}

#[tokio::test]
async fn test_cache_special_characters() {
    let cache = MemoryCache::new(3600);

    // 测试包含特殊字符的键和值
    let special_keys = vec![
        "key with spaces",
        "key-with-dashes",
        "key_with_underscores",
        "key.with.dots",
        "key:with:colons",
        "key/with/slashes",
        "中文键",
        "emoji😀键",
    ];

    for key in &special_keys {
        cache.set(key.to_string(), format!("value for {}", key)).await;
    }

    assert_eq!(cache.len().await, special_keys.len());

    for key in &special_keys {
        let value = cache.get(key).await;
        assert!(value.is_some(), "Key '{}' should exist", key);
        assert_eq!(value.unwrap(), format!("value for {}", key));
    }
}

#[tokio::test]
async fn test_cache_concurrent_access() {
    use std::sync::Arc;
    let cache = Arc::new(MemoryCache::new(3600));

    // 创建多个并发任务
    let mut handles = vec![];

    for i in 1..=100 {
        let cache_clone = Arc::clone(&cache);
        let handle = tokio::spawn(async move {
            cache_clone.set(format!("key{}", i), format!("value{}", i)).await;
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        let _ = handle.await;
    }

    // 验证所有键都被设置
    assert_eq!(cache.len().await, 100);
}

#[tokio::test]
async fn test_cache_empty_string_values() {
    let cache = MemoryCache::new(3600);

    cache.set("empty_key".to_string(), "".to_string()).await;

    let value = cache.get("empty_key").await;
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "");
}

#[tokio::test]
async fn test_cache_update_keeps_entry() {
    let cache = MemoryCache::new(3600);

    cache.set("key1".to_string(), "value1".to_string()).await;

    // 短暂等待
    tokio::time::sleep(Duration::from_millis(100)).await;

    // 更新值
    cache.set("key1".to_string(), "value2".to_string()).await;

    // 立即获取，应该得到新值
    let value = cache.get("key1").await;
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "value2");
}
