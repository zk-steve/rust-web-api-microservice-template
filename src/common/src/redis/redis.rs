use redis_async::{client, resp::RespValue, resp_array};

/// Represents a Redis client for asynchronous operations.
pub struct Redis {
    client: client::PairedConnection,
}

impl Redis {
    /// Creates a new Redis client asynchronously.
    ///
    /// # Arguments
    ///
    /// * `redis_host`: The Redis server hostname or IP address.
    /// * `redis_port`: The Redis server port number.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the created `Redis` client if successful,
    /// or an error if the connection fails.
    pub async fn new(redis_host: &str, redis_port: u16) -> Result<Self, redis_async::error::Error> {
        let client = client::paired_connect(redis_host, redis_port).await?;
        Ok(Self { client })
    }

    /// Sets a key-value pair in Redis asynchronously.
    ///
    /// # Arguments
    ///
    /// * `key`: The key to set in Redis.
    /// * `value`: The value to associate with the key.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the operation.
    pub async fn set(&self, key: &str, value: &str) -> Result<(), redis_async::error::Error> {
        let _: RespValue = self.client.send(resp_array!["SET", key, value]).await?;
        Ok(())
    }

    /// Retrieves a value from Redis asynchronously based on a key.
    ///
    /// # Arguments
    ///
    /// * `key`: The key to look up in Redis.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an optional `String` value if the key is found,
    /// or `None` if the key is not present in Redis, or an error if the operation fails.
    pub async fn get(&self, key: &str) -> Result<Option<String>, redis_async::error::Error> {
        let result: RespValue = self.client.send(resp_array!["GET", key]).await?;
        if let RespValue::BulkString(data) = result {
            Ok(Some(String::from_utf8_lossy(&data).to_string()))
        } else {
            Ok(None)
        }
    }
}
