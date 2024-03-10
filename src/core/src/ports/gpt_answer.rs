use async_trait::async_trait;

use crate::common::errors::CoreError;

#[async_trait]
pub trait GptAnswerPort {
    async fn get_answer(&self, question: &str) -> Result<String, CoreError>;
}
