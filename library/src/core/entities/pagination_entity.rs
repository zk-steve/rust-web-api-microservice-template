use crate::common::errors::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents pagination parameters.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationEntity {
    /// Start index for pagination.
    pub start: usize,
    /// End index for pagination.
    pub end: usize,
    /// Optional sorting criteria.
    pub sort: Option<Vec<String>>,
}

impl PaginationEntity {
    /// Constructs a `PaginationEntity` from query parameters.
    ///
    /// # Arguments
    ///
    /// * `query` - A HashMap containing query parameters.
    ///
    /// # Returns
    ///
    /// A Result containing the constructed `PaginationEntity` or an `Error` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use library::core::entities::pagination_entity::PaginationEntity;
    ///
    /// let mut query_params = HashMap::new();
    /// query_params.insert("start".to_string(), "0".to_string());
    /// query_params.insert("end".to_string(), "10".to_string());
    ///
    /// match PaginationEntity::from_query(&query_params) {
    ///     Ok(pagination_entity) => {
    ///         println!("Parsed pagination entity: {:?}", pagination_entity);
    ///     }
    ///     Err(err) => {
    ///         eprintln!("Failed to parse pagination entity: {}", err);
    ///     }
    /// }
    /// ```
    pub fn from_query(query: &HashMap<String, String>) -> Result<Self, Error> {
        let start = query
            .get("start")
            .unwrap_or(&"0".to_string())
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        let end = query
            .get("end")
            .unwrap_or(&"10".to_string())
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        Ok(PaginationEntity {
            start,
            end,
            sort: None,
        })
    }
}
