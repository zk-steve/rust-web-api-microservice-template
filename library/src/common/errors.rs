
use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;
use warp::http::StatusCode;
use warp::reject::Reject;

use warp::{Rejection, Reply};

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    NotFound,
}

impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        match error {
            Error::NotFound => Ok(warp::reply::with_status(
                "Not found".to_string(),
                StatusCode::NOT_FOUND,
            )),
            Error::ParseError(_) => Ok(warp::reply::with_status(
                "ParseError".to_string(),
                StatusCode::BAD_REQUEST,
            )),
            Error::MissingParameters => Ok(warp::reply::with_status(
                "MissingParameters".to_string(),
                StatusCode::BAD_REQUEST,
            )),
        }
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
