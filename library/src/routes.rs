use crate::controller::QuestionController;
use crate::errors::return_error;
use crate::store::Store;

use warp::http::Method;
use warp::{Filter, Rejection, Reply};

pub struct Router {
    store: Store,
}

impl Router {
    pub fn new(store: Store) -> Self {
        Router { store }
    }
    pub fn routes(self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let store_filter = warp::any().map(move || self.store.clone());
        let cors = warp::cors()
            .allow_any_origin()
            .allow_header("content-type")
            .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
        let get_items = warp::get()
            .and(warp::path("questions"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and(warp::query())
            .and_then(QuestionController::get_questions);

        let add_question = warp::post()
            .and(warp::path("questions"))
            .and(warp::path::end())
            .and(store_filter.clone())
            .and(warp::body::json())
            .and_then(QuestionController::add_question);

        let update_question = warp::put()
            .and(warp::path("questions"))
            .and(store_filter.clone())
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(QuestionController::update_question);

        let delete_question = warp::delete()
            .and(warp::path("questions"))
            .and(store_filter)
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .and_then(QuestionController::delete_question);

        get_items
            .with(cors)
            .or(delete_question)
            .or(update_question)
            .or(add_question)
            .with(warp::trace::request())
            .recover(return_error)
    }
}
