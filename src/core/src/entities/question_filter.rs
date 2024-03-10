use std::collections::HashMap;
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::common::errors::CoreError;
use crate::entities::filter_entity::FilterEntity;
use crate::entities::pagination_entity::PaginationEntity;

/// Represents filters for querying questions.
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionFilter {
    /// Pagination parameters for the query.
    pub pagination: PaginationEntity,
}

/// Implementation of the `TryFrom` trait to convert a HashMap into a `QuestionFilter`.
///
/// This implementation allows converting a HashMap containing query parameters into a `QuestionFilter`.
/// It attempts to parse the pagination parameters from the HashMap and constructs a `QuestionFilter`
/// instance. If parsing fails for any reason, it returns a `CoreError`.
impl TryFrom<HashMap<String, String>> for QuestionFilter {
    type Error = CoreError;

    fn try_from(query: HashMap<String, String>) -> Result<Self, CoreError> {
        Ok(QuestionFilter {
            pagination: PaginationEntity::try_from(query)?,
        })
    }
}

impl FilterEntity<QuestionFilter> for QuestionFilter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_filter_from_query() {
        // Test case 1: Valid query parameters
        let mut query_params_1 = HashMap::new();
        query_params_1.insert("start".to_string(), "0".to_string());
        query_params_1.insert("end".to_string(), "10".to_string());

        match QuestionFilter::try_from(query_params_1) {
            Ok(question_filter) => {
                assert_eq!(question_filter.pagination.start, 0);
                assert_eq!(question_filter.pagination.end, 10);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_question_filter_from_query_parse_error() {
        // Test case 2: Parse parameters error
        let mut query_params_2 = HashMap::new();
        query_params_2.insert("start".to_string(), "asd".to_string());
        query_params_2.insert("end".to_string(), "10".to_string());

        match QuestionFilter::try_from(query_params_2) {
            Ok(_) => {
                panic!("Expected an error, but got Ok");
            }
            Err(err) => match err {
                CoreError::ParseError(_) => {}
                _ => {
                    panic!("Expected ParseError error, but got {:?}", err);
                }
            },
        }
    }
}
