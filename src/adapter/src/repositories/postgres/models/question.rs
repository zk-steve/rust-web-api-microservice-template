use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use rust_core::entities::question::QuestionEntity;
use serde::Serialize;
use std::time::SystemTime;

use super::id::ToId;

#[derive(Debug, Queryable, Serialize, Selectable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name = super::super::schema::questions)]
#[cfg_attr(feature = "postgres", derive(diesel::pg::Pg))]
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
    pub fn from(entity: QuestionEntity) -> Self {
        QuestionModel {
            id: entity.id.to_id(),
            title: entity.title,
            content: entity.content,
            tags: entity.tags.map(|v| v.into_iter().map(Some).collect()),
            created_on: SystemTime::now(),
        }
    }

    pub fn to_entity(self) -> QuestionEntity {
        QuestionEntity {
            id: self.id.to_string().parse().unwrap(),
            title: self.title,
            content: self.content,
            tags: self.tags.map(|v| v.into_iter().flatten().collect()),
        }
    }
}
