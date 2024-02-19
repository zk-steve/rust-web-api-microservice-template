use crate::common::errors::Error;
use crate::core::entities::filter_entity::FilterEntity;
use crate::core::entities::pagination_entity::PaginationEntity;
use std::collections::HashMap;

/// Represents filters for querying questions.
#[derive(Debug)]
pub struct QuestionFilter {
    /// Pagination parameters for the query.
    pub pagination: PaginationEntity,
}

impl QuestionFilter {
    /// Constructs a `QuestionFilter` from query parameters.
    ///
    /// # Arguments
    ///
    /// * `query` - A HashMap containing query parameters.
    ///
    /// # Returns
    ///
    /// A Result containing the constructed `QuestionFilter` or an `Error` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use library::core::entities::question_filter::QuestionFilter;
    ///
    /// let mut query_params = HashMap::new();
    /// query_params.insert("start".to_string(), "0".to_string());
    /// query_params.insert("end".to_string(), "10".to_string());
    ///
    /// match QuestionFilter::from_query(&query_params) {
    ///     Ok(question_filter) => {
    ///         println!("Parsed question filter: {:?}", question_filter);
    ///     }
    ///     Err(err) => {
    ///         eprintln!("Failed to parse question filter: {}", err);
    ///     }
    /// }
    /// ```
    pub fn from_query(query: &HashMap<String, String>) -> Result<Self, Error> {
        Ok(QuestionFilter {
            pagination: PaginationEntity::from_query(query)?,
        })
    }
}

impl FilterEntity<QuestionFilter> for QuestionFilter {}
