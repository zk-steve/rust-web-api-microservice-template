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
    InternalError,
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
            Error::InternalError => Ok(warp::reply::with_status(
                "InternalError".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_return_error_not_found() {
        let rejection = warp::reject::custom(Error::NotFound);
        let response = return_error(rejection).await.unwrap().into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_return_error_parse_error() {
        let parse_error = "ParseError".parse::<i32>().unwrap_err();
        let rejection = warp::reject::custom(Error::ParseError(parse_error));
        let response = return_error(rejection).await.unwrap().into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_return_error_missing_parameters() {
        let rejection = warp::reject::custom(Error::MissingParameters);
        let response = return_error(rejection).await.unwrap().into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_return_error_unknown_rejection() {
        let rejection = warp::reject::reject();
        let response = return_error(rejection).await.unwrap().into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
