use async_trait::async_trait;

use crate::common::errors::Error;
use crate::entities::question::{QuestionEntity, QuestionId};
use crate::entities::question_filter::QuestionFilter;

#[async_trait]
pub trait QuestionPort {
    async fn add(&self, question: QuestionEntity) -> Result<QuestionEntity, Error>;
    async fn update(&self, question: QuestionEntity) -> Result<QuestionEntity, Error>;
    async fn delete(&self, question_id: &QuestionId) -> Result<(), Error>;
    async fn get(&self, question_id: &QuestionId) -> Result<QuestionEntity, Error>;
    async fn list(&self, question_filter: &QuestionFilter) -> Result<Vec<QuestionEntity>, Error>;
}
