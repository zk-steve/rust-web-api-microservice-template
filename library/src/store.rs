use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::errors::Error;
use crate::errors::Error::NotFound;
use crate::question::{Question, QuestionId};
use crate::question_filter::QuestionFilter;

#[derive(Clone, Debug)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Default::default(),
        }
    }

    pub async fn add(&self, question: Question) -> Result<(), Error> {
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question);
        Ok(())
    }

    pub async fn update(&self, question: Question) -> Result<(), Error> {
        self.get(&question.id).await?;
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question);
        Ok(())
    }

    pub async fn delete(&self, question_id: &QuestionId) -> Result<(), Error> {
        self.get(question_id).await?;
        self.questions.write().await.remove(question_id);
        Ok(())
    }

    pub async fn get(&self, question_id: &QuestionId) -> Result<Question, Error> {
        self.questions
            .read()
            .await
            .get(question_id)
            .ok_or(NotFound)
            .cloned()
    }

    pub async fn list(&self, question_filter: &QuestionFilter) -> Result<Vec<Question>, Error> {
        Ok(self
            .questions
            .read()
            .await
            .values()
            .skip(question_filter.pagination.start)
            .take(question_filter.pagination.end - question_filter.pagination.start).cloned()
            .collect::<Vec<Question>>())
    }
}
