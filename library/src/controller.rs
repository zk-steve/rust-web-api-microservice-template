use crate::question::{Question, QuestionId};
use crate::question_filter::QuestionFilter;
use crate::store::Store;
use std::collections::HashMap;

use std::str::FromStr;
use tracing::{instrument};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub struct QuestionController {}

impl QuestionController {
    #[instrument(level = "info", skip(store))]
    pub async fn get_questions(
        store: Store,
        query: HashMap<String, String>,
    ) -> Result<impl Reply, Rejection> {
        let question_filter = QuestionFilter::from_query(&query)?;
        let questions = store.list(&question_filter).await?;
        Ok(warp::reply::json(&questions))
    }

    #[instrument(level = "info", skip(store))]
    pub async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
        store.add(question).await?;
        Ok(warp::reply::with_status("Question added", StatusCode::OK))
    }

    #[instrument(level = "info", skip(store))]
    pub async fn delete_question(store: Store, id: String) -> Result<impl Reply, Rejection> {
        store
            .delete(&QuestionId::from_str(id.as_str()).unwrap())
            .await?;
        Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
    }

    #[instrument(level = "info", skip(store))]
    pub async fn update_question(
        store: Store,
        id: String,
        mut question: Question,
    ) -> Result<impl Reply, Rejection> {
        question.id = QuestionId::from_str(id.as_str()).unwrap();
        store.update(question).await?;
        Ok(warp::reply::with_status("Question updated", StatusCode::OK))
    }
}
