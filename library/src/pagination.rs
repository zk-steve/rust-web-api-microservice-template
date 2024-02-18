use std::collections::HashMap;
use crate::errors::Error;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

impl Pagination {
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
        Ok(Pagination {
            start,
            end,
        })
    }
}