use std::time::Duration;

use async_trait::async_trait;
use redis_async::{client, resp::RespValue, resp_array};

use rust_core::{common::errors::CoreError, ports::cache::CachePort};

/// Represents a Redis cache implementation.
pub struct RedisCache {
    client: client::PairedConnection,
}

impl RedisCache {
    /// Creates a new Redis cache instance.
    ///
    /// # Arguments
    ///
    /// * `redis_host`: Hostname or IP address of the Redis server.
    /// * `redis_port`: Port number of the Redis server.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the initialized `RedisCache` instance.
    pub async fn new(redis_host: &str, redis_port: u16) -> Result<Self, CoreError> {
        client::paired_connect(redis_host, redis_port)
            .await
            .map_err(|err| CoreError::InternalError(err.into()))
            .map(|c| Self { client: c })
    }
}

#[async_trait]
impl CachePort for RedisCache {
    /// Retrieves a value from the Redis cache.
    ///
    /// # Arguments
    ///
    /// * `key`: The key of the value to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the value associated with the key if found.
    async fn get(&self, key: &str) -> Result<String, CoreError> {
        self.client
            .send(resp_array!["GET", key])
            .await
            .map_err(|e| CoreError::InternalError(e.into()))
            .and_then(|r| match r {
                RespValue::BulkString(data) => Ok(String::from_utf8_lossy(&data).to_string()),
                _ => Err(CoreError::NotFound),
            })
    }

    /// Sets a key-value pair in the Redis cache with an optional expiration time.
    ///
    /// # Arguments
    ///
    /// * `key`: The key to set.
    /// * `value`: The value to associate with the key.
    /// * `expiration`: Optional expiration duration for the key-value pair.
    ///
    /// # Returns
    ///
    /// Returns a `Result` where `Ok(())` indicates success (key was set) and `Err(CoreError)` indicates failure.
    async fn set(
        &self,
        key: &str,
        value: &str,
        expiration: Option<Duration>,
    ) -> Result<(), CoreError> {
        let args = vec!["SET", &key, &value];
        let expiration = expiration
            .map(|r| vec!["EX".to_string(), r.as_secs().to_string()])
            .unwrap_or_default();
        self.client
            .send(resp_array![].append(args).append(expiration))
            .await
            .map_err(|e| CoreError::InternalError(e.into()))
            .and_then(|r| match r {
                RespValue::SimpleString(s) if s == "OK" => Ok(()),
                _ => Err(CoreError::NotFound),
            })
    }

    /// Removes a key-value pair from the Redis cache.
    ///
    /// # Arguments
    ///
    /// * `key`: The key to remove.
    ///
    /// # Returns
    ///
    /// Returns a `Result` where `Ok(())` indicates success (key was removed) and `Err(CoreError)` indicates failure.
    async fn del(&mut self, key: &str) -> Result<(), CoreError> {
        self.client
            .send(resp_array!["DEL", key])
            .await
            .map_err(|e| CoreError::InternalError(e.into()))
            .and_then(|resp| match resp {
                RespValue::Integer(num_deleted) => {
                    if num_deleted > 0 {
                        Ok(())
                    } else {
                        Err(CoreError::NotFound)
                    }
                }
                response => Err(CoreError::UnexpectedResponse(format!(
                    "Expect `RespValue::Integer` but found {:?}",
                    response
                ))),
            })
    }
}
