use crate::core::entities::question::QuestionEntity;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Queryable, Serialize, Selectable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = super::super::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QuestionModel {
    pub id: i32,
    /// Title of the question.
    pub title: String,
    /// Content of the question.
    pub content: String,
    /// Optional tags associated with the question.
    pub tags: Option<Vec<Option<String>>>,

    pub created_on: SystemTime,
}

impl QuestionModel {
    pub fn from(entity: QuestionEntity) -> QuestionModel {
        QuestionModel {
            id: entity.id.to_id(),
            title: entity.title,
            content: entity.content,
            tags: Some(vec![]),
            created_on: SystemTime::now(),
        }
    }

    pub fn to_entity(&self) -> QuestionEntity {
        QuestionEntity {
            id: self.id.to_string().parse().unwrap(),
            title: self.title.clone(),
            content: self.content.clone(),
            tags: Some(vec!["".to_string()]),
        }
    }
}
