use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::errors::CoreError;

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

/// Implementation of the `TryFrom` trait to convert a HashMap into a `PaginationEntity`.
///
/// This implementation allows converting a HashMap containing query parameters into a `PaginationEntity`.
/// It attempts to parse the start and end pagination parameters from the HashMap and constructs a `PaginationEntity`
/// instance. If parsing fails for any reason, it returns a `CoreError`.
impl TryFrom<HashMap<String, String>> for PaginationEntity {
    type Error = CoreError;

    fn try_from(query: HashMap<String, String>) -> Result<Self, CoreError> {
        let start = query
            .get("start")
            .unwrap_or(&"0".to_string())
            .parse::<usize>()?;
        let end = query
            .get("end")
            .unwrap_or(&"10".to_string())
            .parse::<usize>()?;
        Ok(PaginationEntity {
            start,
            end,
            sort: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_entity_from_query() {
        // Test case 1: Valid query parameters
        let mut query_params_1 = HashMap::new();
        query_params_1.insert("start".to_string(), "0".to_string());
        query_params_1.insert("end".to_string(), "10".to_string());

        match PaginationEntity::try_from(query_params_1) {
            Ok(pagination_entity) => {
                assert_eq!(pagination_entity.start, 0);
                assert_eq!(pagination_entity.end, 10);
                assert_eq!(pagination_entity.sort, None);
            }
            Err(err) => {
                panic!("Unexpected error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_pagination_entity_from_query_parse_parameters_failed() {
        // Test case 2: Parse parameters failed
        let mut query_params_2 = HashMap::new();
        query_params_2.insert("start".to_string(), "abs".to_string());
        query_params_2.insert("end".to_string(), "10".to_string());
        match PaginationEntity::try_from(query_params_2) {
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
