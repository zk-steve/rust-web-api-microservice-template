use crate::common::errors::return_error;
use crate::controller::{add_question, delete_question, get_questions, update_question};
use crate::core::ports::question::QuestionPort;
use std::sync::Arc;
use warp::http::Method;
use warp::{Filter, Rejection, Reply};

/// Router for handling HTTP requests related to questions.
pub struct Router {
    question_port: Arc<dyn QuestionPort + Send + Sync + 'static>,
}

impl Router {
    /// Creates a new Router instance with the specified QuestionPort.
    pub fn new(question_port: Arc<dyn QuestionPort + Send + Sync + 'static>) -> Self {
        Router {
            question_port: question_port.clone(),
        }
    }

    /// Configures and returns the Warp filter for handling HTTP requests.
    pub fn routes(self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let store_filter = warp::any().map(move || self.question_port.clone());
        let cors = warp::cors()
            .allow_any_origin()
            .allow_header("content-type")
            .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
        let get_items = warp::get()
            .and(warp::path("questions"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and(warp::query())
            .and_then(get_questions);

        let add_question = warp::post()
            .and(warp::path("questions"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and(warp::body::json())
            .and_then(add_question);

        let update_question = warp::put()
            .and(warp::path("questions"))
            .and(store_filter.clone())
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(update_question);

        let delete_question = warp::delete()
            .and(warp::path("questions"))
            .and(store_filter.clone())
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and_then(delete_question);

        get_items
            .with(cors)
            .or(delete_question)
            .or(update_question)
            .or(add_question)
            .with(warp::trace::request())
            .recover(return_error)
    }
}
