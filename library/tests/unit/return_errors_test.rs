#[cfg(test)]
mod tests {
    use library::common::errors::{return_error, Error};
    use warp::http::StatusCode;
    use warp::reply::Reply;

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
