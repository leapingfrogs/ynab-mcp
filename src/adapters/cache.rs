//! Simple in-memory cache for YNAB API responses to improve performance.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A simple time-based cache entry.
#[derive(Debug, Clone)]
struct CacheEntry {
    data: serde_json::Value,
    created_at: Instant,
    ttl: Duration,
}

impl CacheEntry {
    fn new(data: serde_json::Value, ttl: Duration) -> Self {
        Self {
            data,
            created_at: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

/// Simple in-memory cache for API responses with TTL support.
///
/// This cache helps reduce API calls to the YNAB service by storing
/// responses for a configurable time period.
#[derive(Debug)]
pub struct ApiResponseCache {
    entries: HashMap<String, CacheEntry>,
    default_ttl: Duration,
}

impl ApiResponseCache {
    /// Creates a new API response cache with default TTL of 5 minutes.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::adapters::cache::ApiResponseCache;
    ///
    /// let cache = ApiResponseCache::new();
    /// assert_eq!(cache.size(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Creates a new API response cache with custom default TTL.
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::adapters::cache::ApiResponseCache;
    /// use std::time::Duration;
    ///
    /// let cache = ApiResponseCache::with_ttl(Duration::from_secs(60));
    /// assert_eq!(cache.size(), 0);
    /// ```
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl: ttl,
        }
    }

    /// Stores a response in the cache with the default TTL.
    ///
    /// # Arguments
    /// * `key` - The cache key (usually the API endpoint path)
    /// * `data` - The JSON response data to cache
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::adapters::cache::ApiResponseCache;
    /// use serde_json::json;
    ///
    /// let mut cache = ApiResponseCache::new();
    /// let data = json!({"budget_id": "123", "name": "My Budget"});
    /// cache.set("/budgets/123", data);
    /// assert_eq!(cache.size(), 1);
    /// ```
    pub fn set(&mut self, key: &str, data: serde_json::Value) {
        self.set_with_ttl(key, data, self.default_ttl);
    }

    /// Stores a response in the cache with a custom TTL.
    ///
    /// # Arguments
    /// * `key` - The cache key
    /// * `data` - The JSON response data to cache
    /// * `ttl` - Time-to-live for this specific entry
    pub fn set_with_ttl(&mut self, key: &str, data: serde_json::Value, ttl: Duration) {
        let entry = CacheEntry::new(data, ttl);
        self.entries.insert(key.to_string(), entry);
    }

    /// Retrieves a response from the cache if it exists and hasn't expired.
    ///
    /// # Arguments
    /// * `key` - The cache key to lookup
    ///
    /// # Returns
    /// * `Some(data)` if the key exists and hasn't expired
    /// * `None` if the key doesn't exist or has expired
    ///
    /// # Example
    /// ```
    /// use ynab_mcp::adapters::cache::ApiResponseCache;
    /// use serde_json::json;
    ///
    /// let mut cache = ApiResponseCache::new();
    /// let data = json!({"budget_id": "123", "name": "My Budget"});
    /// cache.set("/budgets/123", data.clone());
    ///
    /// let cached = cache.get("/budgets/123");
    /// assert_eq!(cached, Some(data));
    /// ```
    pub fn get(&mut self, key: &str) -> Option<serde_json::Value> {
        if let Some(entry) = self.entries.get(key) {
            if !entry.is_expired() {
                return Some(entry.data.clone());
            } else {
                // Remove expired entry
                self.entries.remove(key);
            }
        }
        None
    }

    /// Removes all expired entries from the cache.
    pub fn cleanup_expired(&mut self) {
        self.entries.retain(|_, entry| !entry.is_expired());
    }

    /// Returns the number of entries currently in the cache.
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Clears all entries from the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for ApiResponseCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::time::Duration;

    #[test]
    fn should_create_new_cache() {
        let cache = ApiResponseCache::new();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn should_create_cache_with_custom_ttl() {
        let cache = ApiResponseCache::with_ttl(Duration::from_secs(60));
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn should_store_and_retrieve_data() {
        let mut cache = ApiResponseCache::new();
        let data = json!({"budget_id": "123", "name": "My Budget"});

        cache.set("/budgets/123", data.clone());

        assert_eq!(cache.size(), 1);
        let retrieved = cache.get("/budgets/123");
        assert_eq!(retrieved, Some(data));
    }

    #[test]
    fn should_return_none_for_missing_key() {
        let mut cache = ApiResponseCache::new();

        let result = cache.get("/missing/key");
        assert_eq!(result, None);
    }

    #[test]
    fn should_handle_expired_entries() {
        let mut cache = ApiResponseCache::with_ttl(Duration::from_millis(1));
        let data = json!({"test": "data"});

        cache.set("/test", data);
        assert_eq!(cache.size(), 1);

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(10));

        let result = cache.get("/test");
        assert_eq!(result, None);
        assert_eq!(cache.size(), 0); // Should be removed after get
    }

    #[test]
    fn should_set_with_custom_ttl() {
        let mut cache = ApiResponseCache::new();
        let data = json!({"custom": "ttl"});

        cache.set_with_ttl("/custom", data.clone(), Duration::from_secs(1));

        let retrieved = cache.get("/custom");
        assert_eq!(retrieved, Some(data));
    }

    #[test]
    fn should_cleanup_expired_entries() {
        let mut cache = ApiResponseCache::with_ttl(Duration::from_millis(1));
        let data1 = json!({"id": 1});
        let data2 = json!({"id": 2});

        cache.set("/key1", data1);
        cache.set_with_ttl("/key2", data2.clone(), Duration::from_secs(60)); // Long TTL

        assert_eq!(cache.size(), 2);

        std::thread::sleep(Duration::from_millis(10)); // Wait for first to expire

        cache.cleanup_expired();
        assert_eq!(cache.size(), 1);

        // Key2 should still be available
        let result = cache.get("/key2");
        assert_eq!(result, Some(data2));
    }

    #[test]
    fn should_clear_all_entries() {
        let mut cache = ApiResponseCache::new();
        cache.set("/key1", json!({"id": 1}));
        cache.set("/key2", json!({"id": 2}));

        assert_eq!(cache.size(), 2);

        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn should_support_default_trait() {
        let cache: ApiResponseCache = Default::default();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn should_handle_cache_entry_debug_format() {
        let entry = CacheEntry::new(json!({"test": "data"}), Duration::from_secs(60));
        let debug_str = format!("{:?}", entry);
        assert!(debug_str.contains("CacheEntry"));
        assert!(debug_str.contains("test"));
    }
}
