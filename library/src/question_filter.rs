use std::collections::HashMap;
use crate::errors::Error;
use crate::pagination::Pagination;

#[derive(Debug)]
pub struct QuestionFilter {
    pub pagination: Pagination,
}

impl QuestionFilter {
    pub fn from_query(query: &HashMap<String, String>) -> Result<Self, Error> {
        Ok(QuestionFilter {
            pagination: Pagination::from_query(query)?,
        })
    }
}