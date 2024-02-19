use crate::common::errors::Error;
use crate::common::errors::Error::NotFound;
use crate::core::entities::question::{QuestionEntity, QuestionId};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::core::entities::question_filter::QuestionFilter;
use crate::core::ports::question::QuestionPort;

#[derive(Clone, Debug)]
pub struct QuestionDBRepository {
    pub questions: Arc<RwLock<HashMap<QuestionId, QuestionEntity>>>,
}

impl Default for QuestionDBRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl QuestionDBRepository {
    pub fn new() -> Self {
        QuestionDBRepository {
            questions: Default::default(),
        }
    }
}

#[async_trait]
impl QuestionPort for QuestionDBRepository {
    async fn add(&self, question: QuestionEntity) -> Result<(), Error> {
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question);
        Ok(())
    }

    async fn update(&self, question: QuestionEntity) -> Result<(), Error> {
        self.get(&question.id).await?;
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question);
        Ok(())
    }

    async fn delete(&self, question_id: &QuestionId) -> Result<(), Error> {
        self.get(question_id).await?;
        self.questions.write().await.remove(question_id);
        Ok(())
    }

    async fn get(&self, question_id: &QuestionId) -> Result<QuestionEntity, Error> {
        self.questions
            .read()
            .await
            .get(question_id)
            .ok_or(NotFound)
            .cloned()
    }

    async fn list(&self, question_filter: &QuestionFilter) -> Result<Vec<QuestionEntity>, Error> {
        Ok(self
            .questions
            .read()
            .await
            .values()
            .skip(question_filter.pagination.start)
            .take(question_filter.pagination.end - question_filter.pagination.start)
            .cloned()
            .collect::<Vec<_>>())
    }
}
