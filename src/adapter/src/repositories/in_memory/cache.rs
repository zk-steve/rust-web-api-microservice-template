use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};

use async_trait::async_trait;
use tokio::sync::RwLock;

use rust_core::common::errors::CoreError;
use rust_core::ports::cache::CachePort;

/// Represents an in-memory cache implementation.
pub struct InMemoryCache {
    cache: Arc<RwLock<HashMap<String, (String, Option<SystemTime>)>>>,
}

impl Default for InMemoryCache {
    /// Creates a new default instance of in-memory cache.
    fn default() -> Self {
        Self {
            cache: Arc::new(RwLock::default()),
        }
    }
}

impl InMemoryCache {
    /// Removes expired entries from the cache asynchronously.
    ///
    /// This method is used to remove any expired cache entries based on their expiration time.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the cleanup is successful, otherwise returns a `CoreError`.
    async fn cleanup_expired_entries(&self) -> Result<(), CoreError> {
        let mut cache = self.cache.write().await;
        let now = SystemTime::now();
        cache.retain(|_, (_, expiry_time)| match expiry_time {
            Some(exp) => *exp > now,
            None => true, // No expiration time, don't remove
        });
        Ok(())
    }
}

#[async_trait]
impl CachePort for InMemoryCache {
    /// Retrieves a value from the cache based on the provided key.
    ///
    /// # Arguments
    ///
    /// * `key`: A string representing the key to retrieve from the cache.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `value` if the key is found and not expired,
    /// or throw `CoreError::NotFound` if the key is not found or expired.
    async fn get(&self, key: &str) -> Result<String, CoreError> {
        let cache = self.cache.read().await;
        match cache.get(key) {
            Some((value, expiry_time))
                if expiry_time.map_or(true, |exp| exp > SystemTime::now()) =>
            {
                Ok(value.clone())
            }
            _ => Err(CoreError::NotFound),
        }
    }

    /// Sets a key-value pair in the cache with an optional expiration duration.
    ///
    /// # Arguments
    ///
    /// * `key`: A string representing the key to set in the cache.
    /// * `value`: A string representing the value to associate with the key.
    /// * `expiration`: An optional `Duration` specifying the expiration time for the key-value pair.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the set operation is successful, otherwise returns a `CoreError`.
    async fn set(
        &self,
        key: &str,
        value: &str,
        expiration: Option<Duration>,
    ) -> Result<(), CoreError> {
        self.cleanup_expired_entries().await?;

        // Calculate expiry_time based on expiration duration
        let expiry_time = expiration.map(|exp| SystemTime::now() + exp);

        self.cache
            .write()
            .await
            .insert(key.to_string(), (value.to_string(), expiry_time));
        Ok(())
    }

    /// Removes a key-value pair from the cache based on the provided key.
    ///
    /// # Arguments
    ///
    /// * `key`: A string representing the key to remove from the cache.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the key is found and removed, otherwise returns a `CoreError`.
    async fn del(&mut self, key: &str) -> Result<(), CoreError> {
        self.cache
            .write()
            .await
            .remove(key)
            .map(|_| ())
            .ok_or(CoreError::NotFound)
    }
}
