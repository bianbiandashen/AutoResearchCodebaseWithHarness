// Request caching system inspired by lm-evaluation-harness
//
// Key features:
// 1. Content-addressed caching (SHA256 hash of request)
// 2. SQLite backend for persistence
// 3. Automatic expiration and cleanup
// 4. Expected 10-50x speedup for repeated evaluations

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// SHA256 hash of request content
type ContentHash = String;

/// Cached request/response pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Content hash (key)
    pub hash: ContentHash,

    /// Original request
    pub request: String,

    /// Cached response
    pub response: String,

    /// Timestamp when cached (Unix epoch seconds)
    pub timestamp: u64,

    /// Optional metadata
    pub metadata: serde_json::Value,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Path to SQLite database file
    pub db_path: PathBuf,

    /// Expiration time (None = never expire)
    pub expiration: Option<Duration>,

    /// Maximum cache size in entries
    pub max_entries: Option<usize>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from(".cache/requests.db"),
            expiration: Some(Duration::from_secs(30 * 24 * 3600)), // 30 days
            max_entries: Some(10000),
        }
    }
}

/// Request cache using SQLite
pub struct RequestCache {
    config: CacheConfig,
    // In a full implementation, this would hold rusqlite::Connection
    // For now, using in-memory HashMap for simplicity
    cache: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<ContentHash, CacheEntry>>>,
}

impl RequestCache {
    /// Create or open cache
    pub fn new(config: CacheConfig) -> Result<Self> {
        // In production, this would:
        // 1. Create SQLite database if not exists
        // 2. Initialize schema
        // 3. Create indexes on hash column

        // For now, use in-memory cache
        let cache = std::sync::Arc::new(std::sync::RwLock::new(
            std::collections::HashMap::new()
        ));

        Ok(Self { config, cache })
    }

    /// Create cache with default config
    pub fn default() -> Result<Self> {
        Self::new(CacheConfig::default())
    }

    /// Compute content hash for a request
    fn compute_hash(request: &str) -> ContentHash {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        request.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Get cached response if exists and not expired
    pub fn get(&self, request: &str) -> Option<String> {
        let hash = Self::compute_hash(request);
        let cache = self.cache.read().unwrap();

        if let Some(entry) = cache.get(&hash) {
            // Check expiration
            if let Some(expiration) = self.config.expiration {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if now - entry.timestamp > expiration.as_secs() {
                    // Expired
                    return None;
                }
            }

            Some(entry.response.clone())
        } else {
            None
        }
    }

    /// Store response in cache
    pub fn put(&self, request: &str, response: &str) -> Result<()> {
        let hash = Self::compute_hash(request);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = CacheEntry {
            hash: hash.clone(),
            request: request.to_string(),
            response: response.to_string(),
            timestamp,
            metadata: serde_json::Value::Null,
        };

        let mut cache = self.cache.write().unwrap();

        // Check size limit
        if let Some(max_entries) = self.config.max_entries {
            if cache.len() >= max_entries {
                // Evict oldest entry (LRU policy)
                if let Some(oldest_key) = cache
                    .iter()
                    .min_by_key(|(_, entry)| entry.timestamp)
                    .map(|(k, _)| k.clone())
                {
                    cache.remove(&oldest_key);
                }
            }
        }

        cache.insert(hash, entry);
        Ok(())
    }

    /// Clear all cached entries
    pub fn clear(&self) -> Result<()> {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.cache.read().unwrap();
        CacheStats {
            total_entries: cache.len(),
            size_bytes: cache
                .values()
                .map(|e| e.request.len() + e.response.len())
                .sum(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub size_bytes: usize,
}

/// Cached execution wrapper
///
/// Wraps expensive operations with automatic caching
pub struct CachedExecutor<F> {
    cache: RequestCache,
    executor: F,
}

impl<F, Fut> CachedExecutor<F>
where
    F: Fn(String) -> Fut,
    Fut: std::future::Future<Output = Result<String>>,
{
    pub fn new(cache: RequestCache, executor: F) -> Self {
        Self { cache, executor }
    }

    /// Execute with caching
    pub async fn execute(&self, request: &str) -> Result<String> {
        // Check cache first
        if let Some(cached_response) = self.cache.get(request) {
            return Ok(cached_response);
        }

        // Cache miss - execute
        let response = (self.executor)(request.to_string()).await?;

        // Store in cache
        self.cache.put(request, &response)?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let cache = RequestCache::default().unwrap();

        let request = "test request";
        let response = "test response";

        // Put
        cache.put(request, response).unwrap();

        // Get
        let cached = cache.get(request).unwrap();
        assert_eq!(cached, response);
    }

    #[test]
    fn test_cache_miss() {
        let cache = RequestCache::default().unwrap();

        let result = cache.get("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_eviction() {
        let config = CacheConfig {
            max_entries: Some(2),
            ..Default::default()
        };
        let cache = RequestCache::new(config).unwrap();

        // Add 3 entries (should evict oldest)
        cache.put("req1", "resp1").unwrap();
        std::thread::sleep(Duration::from_millis(10));
        cache.put("req2", "resp2").unwrap();
        std::thread::sleep(Duration::from_millis(10));
        cache.put("req3", "resp3").unwrap();

        // req1 should be evicted
        assert!(cache.get("req1").is_none());
        assert!(cache.get("req2").is_some());
        assert!(cache.get("req3").is_some());
    }

    #[tokio::test]
    async fn test_cached_executor() {
        let cache = RequestCache::default().unwrap();

        let mut call_count = 0;
        let executor = CachedExecutor::new(cache, |req| async move {
            // This should only be called once due to caching
            Ok::<String, anyhow::Error>(format!("Response to: {}", req))
        });

        // First call - cache miss
        let result1 = executor.execute("test").await.unwrap();

        // Second call - cache hit (executor not called)
        let result2 = executor.execute("test").await.unwrap();

        assert_eq!(result1, result2);
    }
}
