// AvilaCache - Native Caching System
// Zero External Dependencies 🦀

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
struct CacheEntry {
    value: Vec<u8>,
    expires_at: Option<SystemTime>,
}

impl CacheEntry {
    fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            SystemTime::now() > expires_at
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Cache {
    data: HashMap<String, CacheEntry>,
    max_size: usize,
}

impl Cache {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
        }
    }

    pub fn set(&mut self, key: String, value: Vec<u8>, ttl: Option<Duration>) {
        // Evict if at capacity
        if self.data.len() >= self.max_size && !self.data.contains_key(&key) {
            self.evict_oldest();
        }

        let expires_at = ttl.map(|d| SystemTime::now() + d);

        self.data.insert(
            key,
            CacheEntry {
                value,
                expires_at,
            },
        );
    }

    pub fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        if let Some(entry) = self.data.get(key) {
            if entry.is_expired() {
                self.data.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        } else {
            None
        }
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn evict_oldest(&mut self) {
        if let Some(key) = self.data.keys().next().cloned() {
            self.data.remove(&key);
        }
    }

    pub fn cleanup_expired(&mut self) {
        let expired_keys: Vec<String> = self
            .data
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            self.data.remove(&key);
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new(1000)
    }
}

// TTL Helpers
pub mod ttl {
    use std::time::Duration;

    pub const fn seconds(n: u64) -> Duration {
        Duration::from_secs(n)
    }

    pub const fn minutes(n: u64) -> Duration {
        Duration::from_secs(n * 60)
    }

    pub const fn hours(n: u64) -> Duration {
        Duration::from_secs(n * 3600)
    }

    pub const fn days(n: u64) -> Duration {
        Duration::from_secs(n * 86400)
    }
}

// String value helpers
pub mod string {
    use super::*;

    pub fn set_str(cache: &mut Cache, key: String, value: String, ttl: Option<Duration>) {
        cache.set(key, value.into_bytes(), ttl);
    }

    pub fn get_str(cache: &mut Cache, key: &str) -> Option<String> {
        cache.get(key).and_then(|bytes| String::from_utf8(bytes).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_create() {
        let cache = Cache::new(100);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_set_get() {
        let mut cache = Cache::new(100);
        cache.set("key1".to_string(), b"value1".to_vec(), None);

        let value = cache.get("key1");
        assert_eq!(value, Some(b"value1".to_vec()));
    }

    #[test]
    fn test_ttl_expiration() {
        let mut cache = Cache::new(100);
        cache.set(
            "temp".to_string(),
            b"data".to_vec(),
            Some(Duration::from_millis(100)),
        );

        assert!(cache.get("temp").is_some());
        thread::sleep(Duration::from_millis(150));
        assert!(cache.get("temp").is_none());
    }

    #[test]
    fn test_delete() {
        let mut cache = Cache::new(100);
        cache.set("key".to_string(), b"val".to_vec(), None);

        assert!(cache.delete("key"));
        assert!(cache.get("key").is_none());
        assert!(!cache.delete("key"));
    }

    #[test]
    fn test_clear() {
        let mut cache = Cache::new(100);
        cache.set("k1".to_string(), b"v1".to_vec(), None);
        cache.set("k2".to_string(), b"v2".to_vec(), None);

        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_max_size() {
        let mut cache = Cache::new(2);
        cache.set("k1".to_string(), b"v1".to_vec(), None);
        cache.set("k2".to_string(), b"v2".to_vec(), None);
        cache.set("k3".to_string(), b"v3".to_vec(), None);

        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_string_helpers() {
        let mut cache = Cache::new(100);
        string::set_str(&mut cache, "name".to_string(), "Dubai".to_string(), None);

        let value = string::get_str(&mut cache, "name");
        assert_eq!(value, Some("Dubai".to_string()));
    }

    #[test]
    fn test_cleanup_expired() {
        let mut cache = Cache::new(100);
        cache.set(
            "short".to_string(),
            b"data".to_vec(),
            Some(Duration::from_millis(50)),
        );
        cache.set("long".to_string(), b"data".to_vec(), None);

        thread::sleep(Duration::from_millis(100));
        cache.cleanup_expired();

        assert_eq!(cache.len(), 1);
        assert!(cache.get("long").is_some());
    }
}
