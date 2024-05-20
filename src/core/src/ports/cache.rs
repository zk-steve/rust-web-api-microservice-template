use async_trait::async_trait;

use std::time::Duration;

use crate::common::errors::CoreError;

/// Represents a cache port for storing key-value pairs.
#[async_trait]
pub trait CachePort {
    /// Retrieves a value from the cache based on the given key.
    ///
    /// # Arguments
    ///
    /// * `key`: The key to look up in the cache.
    ///
    /// # Returns
    ///
    /// Returns an String value associated with the key, or `CoreError` if the key is not present in the cache.
    async fn get(&self, key: &str) -> Result<String, CoreError>;

    /// Sets a key-value pair in the cache with an optional expiration duration.
    ///
    /// # Arguments
    ///
    /// * `key`: The key to set in the cache.
    /// * `value`: The value to associate with the key.
    /// * `expiration`: Optional expiration duration for the key-value pair.
    ///
    /// # Returns
    ///
    /// Returns `Result(())` if the key-value pair is successfully set in the cache, `false` otherwise.
    async fn set(
        &self,
        key: &str,
        value: &str,
        expiration: Option<Duration>,
    ) -> Result<(), CoreError>;

    /// Removes a key-value pair from the cache based on the given key.
    ///
    /// # Arguments
    ///
    /// * `key`: The key to remove from the cache.
    ///
    /// # Returns
    ///
    /// Returns `Result(())` if the key-value pair is successfully removed from the cache, `false` otherwise.
    async fn del(&mut self, key: &str) -> Result<(), CoreError>;
}
