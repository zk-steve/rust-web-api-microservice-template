use crate::core::entities::question_filter::QuestionFilter;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tracing::instrument;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::core::entities::question::{QuestionEntity, QuestionId};
use crate::core::ports::question::QuestionPort;

/// Handler for retrieving questions based on query parameters.
///
/// This function retrieves questions based on the provided query parameters. It takes a HashMap
/// containing the query parameters and a reference to the QuestionPort trait object. It returns
/// a JSON response containing the list of questions.
#[instrument(level = "info", skip(question_port))]
pub async fn get_questions(
    question_port: Arc<dyn QuestionPort + Send + Sync>,
    query: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let question_filter = QuestionFilter::from_query(&query)?;
    let questions = question_port.list(&question_filter).await?;
    Ok(warp::reply::json(&questions))
}

/// Handler for retrieving a question by ID.
///
/// This function retrieves a question with the specified ID from the system. It takes the ID of
/// the question to be got as a string and a reference to the QuestionPort trait object. It returns
/// a JSON response containing the question.
#[instrument(level = "info", skip(question_port))]
pub async fn get_question(
    question_port: Arc<dyn QuestionPort + Send + Sync>,
    id: String,
) -> Result<impl Reply, Rejection> {
    let question = question_port
        .get(&QuestionId::from_str(id.as_str()).unwrap())
        .await?;
    Ok(warp::reply::json(&question))
}

/// Handler for adding a new question.
///
/// This function adds a new question to the system. It takes a QuestionEntity representing the
/// question to be added and a reference to the QuestionPort trait object. It returns a success
/// response with status code 200 if the question is added successfully.
#[instrument(level = "info", skip(question_port))]
pub async fn add_question(
    question_port: Arc<dyn QuestionPort + Send + Sync>,
    question: QuestionEntity,
) -> Result<impl Reply, Rejection> {
    question_port.add(question).await?;
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

/// Handler for deleting a question by ID.
///
/// This function deletes a question with the specified ID from the system. It takes the ID of
/// the question to be deleted as a string and a reference to the QuestionPort trait object. It
/// returns a success response with status code 200 if the question is deleted successfully.
#[instrument(level = "info", skip(question_port))]
pub async fn delete_question(
    question_port: Arc<dyn QuestionPort + Send + Sync>,
    id: String,
) -> Result<impl Reply, Rejection> {
    question_port
        .delete(&QuestionId::from_str(id.as_str()).unwrap())
        .await?;
    Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
}

/// Handler for updating a question by ID.
///
/// This function updates a question with the specified ID in the system. It takes the ID of the
/// question to be updated as a string, the updated QuestionEntity, and a reference to the
/// QuestionPort trait object. It returns a success response with status code 200 if the question
/// is updated successfully.
#[instrument(level = "info", skip(question_port))]
pub async fn update_question(
    question_port: Arc<dyn QuestionPort + Send + Sync>,
    id: String,
    mut question: QuestionEntity,
) -> Result<impl Reply, Rejection> {
    question.id = QuestionId::from_str(id.as_str()).unwrap();
    question_port.update(question).await?;
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}
