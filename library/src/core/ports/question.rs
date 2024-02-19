use crate::common::errors::Error;
use crate::core::entities::question::{QuestionEntity, QuestionId};
use crate::core::entities::question_filter::QuestionFilter;
use async_trait::async_trait;

#[async_trait]
pub trait QuestionPort {
    async fn add(&self, question: QuestionEntity) -> Result<(), Error>;
    async fn update(&self, question: QuestionEntity) -> Result<(), Error>;
    async fn delete(&self, question_id: &QuestionId) -> Result<(), Error>;
    async fn get(&self, question_id: &QuestionId) -> Result<QuestionEntity, Error>;
    async fn list(&self, question_filter: &QuestionFilter) -> Result<Vec<QuestionEntity>, Error>;
}
