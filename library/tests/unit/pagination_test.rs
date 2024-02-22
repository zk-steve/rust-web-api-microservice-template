use library::common::errors::Error;
use library::core::entities::pagination_entity::PaginationEntity;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_entity_from_query() {
        // Test case 1: Valid query parameters
        let mut query_params_1 = HashMap::new();
        query_params_1.insert("start".to_string(), "0".to_string());
        query_params_1.insert("end".to_string(), "10".to_string());

        match PaginationEntity::from_query(&query_params_1) {
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

        match PaginationEntity::from_query(&query_params_2) {
            Ok(_) => {
                panic!("Expected an error, but got Ok");
            }
            Err(err) => {
                match err {
                    Error::ParseError(_) => {}
                    _ => {
                        panic!("Expected ParseError error, but got {:?}", err);
                    }
                }
            }
        }
    }
}
