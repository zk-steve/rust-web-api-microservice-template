#[cfg(test)]
mod tests {
    use library::core::entities::question_filter::QuestionFilter;
    use std::collections::HashMap;
    use library::common::errors::Error;

    #[test]
    fn test_question_filter_from_query() {
        // Test case 1: Valid query parameters
        let mut query_params_1 = HashMap::new();
        query_params_1.insert("start".to_string(), "0".to_string());
        query_params_1.insert("end".to_string(), "10".to_string());

        match QuestionFilter::from_query(&query_params_1) {
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

        match QuestionFilter::from_query(&query_params_2) {
            Ok(_) => {
                panic!("Expected an error, but got Ok");
            }
            Err(err) => {
                match err {
                    Error::ParseError(_) => {}
                    _ => {
                        panic!("Expected MissingParameters error, but got {:?}", err);
                    }
                }
            }
        }
    }
}
