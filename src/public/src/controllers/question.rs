use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use tracing::instrument;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::Reply;

use adapter::repositories::grpc::gpt_answer_client::GptAnswerClient;
use rust_core::entities::question::{QuestionEntity, QuestionId};
use rust_core::entities::question_filter::QuestionFilter;
use rust_core::ports::gpt_answer::GptAnswerPort;
use rust_core::ports::question::QuestionPort;

use crate::errors::WarpError;

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
    let question_filter = QuestionFilter::try_from(query).map_err(WarpError::from)?;

    let questions = question_port
        .list(&question_filter)
        .await
        .map_err(WarpError::from)?;

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
    let question_id = QuestionId::from_str(id.as_str()).map_err(WarpError::from)?;

    let question = question_port
        .get(&question_id)
        .await
        .map_err(WarpError::from)?;

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
    question_port.add(question).await.map_err(WarpError::from)?;

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
    let question_id = QuestionId::from_str(id.as_str()).map_err(WarpError::from)?;

    question_port
        .delete(&question_id)
        .await
        .map_err(WarpError::from)?;

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
    question.id = QuestionId::from_str(id.as_str()).map_err(WarpError::from)?;

    question_port
        .update(question)
        .await
        .map_err(WarpError::from)?;

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

/// Controller for handling HTTP GET requests to fetch answers for a given question ID.
///
/// This controller retrieves a question from the provided `QuestionPort` based on the
/// specified ID, calls the gRPC client (`GptAnswerClient`) to get an answer using the
/// question's content, and responds with the answer in a JSON format.
///
/// # Arguments
///
/// * `question_port`: A trait object implementing `QuestionPort` for interacting with questions.
/// * `gpt_answer_client`: The gRPC client instance for answering questions.
/// * `id`: The ID of the question to fetch the answer for.
///
/// # Returns
///
/// Returns a `Result` containing the HTTP response. If successful, responds with the answer
/// as a JSON string and a status code of `200 OK`. If there's an error during
/// question retrieval, gRPC communication, or response construction, it returns a Warp `Rejection`.
#[instrument(level = "info", skip(question_port, gpt_answer_client))]
pub async fn get_question_answer(
    question_port: Arc<dyn QuestionPort + Send + Sync>,
    mut gpt_answer_client: GptAnswerClient,
    id: String,
) -> Result<impl Reply, Rejection> {
    let question_id = QuestionId::from_str(&id).map_err(WarpError::from)?;

    let question = question_port
        .get(&question_id)
        .await
        .map_err(WarpError::from)?;

    gpt_answer_client.connect().await.map_err(WarpError::from)?;

    let answer = gpt_answer_client
        .get_answer(&question.content)
        .await
        .map_err(WarpError::from)?;

    Ok(warp::reply::with_status(answer, StatusCode::OK))
}
