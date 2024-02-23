use deadpool_diesel::{
    postgres::{Pool, Runtime},
    Manager,
};
use library::{
    adapter::repositories::{
        in_memory::question::QuestionInMemoryRepository,
        postgres::question_db::QuestionDBRepository,
    },
    core::{
        entities::question::{QuestionEntity, QuestionId},
        ports::question::QuestionPort,
    },
    routes::Router,
};
use serde_json::Value;
use std::{str::FromStr, sync::Arc}; // Import Value from serde_json

#[tokio::test]
async fn questions_router_in_memory_test() {
    use warp::http::StatusCode;
    use warp::test::request;

    // Set up an in-memory question port for testing
    let question_port: Arc<dyn QuestionPort + Send + Sync> =
        Arc::new(QuestionInMemoryRepository::default());

    // Create a Router instance with the in-memory question port
    let router = Router::new(question_port.clone());

    let raw_question_id = String::from("question-1");
    let question_id = QuestionId::from_str(&raw_question_id.clone()).unwrap();

    let question = QuestionEntity {
        id: question_id.clone(),
        title: "How to write integration tests in Rust?".to_string(),
        content: "I'm trying to write integration tests for a Warp router, but I'm not sure how to do it. Can someone help me?".to_string(),
        tags: Some(vec!["rust".to_string(), "warp".to_string()]),
    };

    // Test POST /questions to create a new question
    let post_resp = request()
        .method("POST")
        .path("/questions")
        .json(&question)
        .reply(&router.clone().routes())
        .await;

    assert_eq!(post_resp.status(), StatusCode::OK);

    // Test GET /questions to get the created question
    let get_resp = request()
        .method("GET")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .reply(&router.clone().routes())
        .await;

    assert_eq!(get_resp.status(), StatusCode::OK);

    // Deserialize the response body into a Value
    let response_body: Value =
        serde_json::from_slice(&get_resp.body()).expect("Failed to deserialize response body");
    let get_question: QuestionEntity = serde_json::from_value(response_body)
        .expect("Failed to deserialize response body into QuestionEntity");

    assert_eq!(get_question, question);

    // Test PUT /questions to update the created question
    let updated_question = QuestionEntity {
        id: question_id.clone(),
        title: "How to write integration tests in Rust?".to_string(),
        content: "I'm trying to write integration tests for a Warp router, but I'm not sure how to do it. Can someone help me?".to_string(),
        tags: Some(vec!["rust".to_string(), "warp".to_string(), "testing".to_string()]),
    };

    let put_resp = request()
        .method("PUT")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .json(&updated_question)
        .reply(&router.clone().routes())
        .await;

    assert_eq!(put_resp.status(), StatusCode::OK);

    // Test GET /questions to get the updated question
    let get_updated_resp = request()
        .method("GET")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .reply(&router.clone().routes())
        .await;

    assert_eq!(get_updated_resp.status(), StatusCode::OK);

    // Deserialize the response body into a Value
    let updated_response_body: Value = serde_json::from_slice(&get_updated_resp.body())
        .expect("Failed to deserialize response body");
    let get_updated_question: QuestionEntity = serde_json::from_value(updated_response_body)
        .expect("Failed to deserialize response body into QuestionEntity");

    assert_eq!(get_updated_question, updated_question);

    // Test DELETE /questions to delete the created question
    let delete_resp = request()
        .method("DELETE")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .reply(&router.clone().routes())
        .await;

    assert_eq!(delete_resp.status(), StatusCode::OK);

    // Test GET /questions to get the deleted question
    let get_deleted_resp = request()
        .method("GET")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .reply(&router.clone().routes())
        .await;

    assert_eq!(get_deleted_resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn questions_router_postgres_db_test() {
    use warp::http::StatusCode;
    use warp::test::request;

    struct DatabaseConfig {
        url: String,
        max_size: usize,
    }

    let database_config = DatabaseConfig {
        url: "postgres://postgres:changeme@127.0.0.1:5432/postgres".to_string(),
        max_size: 10,
    };
    let manager = Manager::new(database_config.url, Runtime::Tokio1);

    // Set up a postgres database question port for testing
    let db_pool = Pool::builder(manager)
        .max_size(database_config.max_size)
        .build()
        .unwrap();
    let question_port: Arc<dyn QuestionPort + Send + Sync> =
        Arc::new(QuestionDBRepository::new(db_pool.clone()));

    // Create a Router instance with the in-memory question port
    let router = Router::new(question_port.clone());

    let raw_question_id = String::from("question-1");
    let question_id = QuestionId::from_str(&raw_question_id.clone()).unwrap();

    let question = QuestionEntity {
        id: question_id.clone(),
        title: "How to write integration tests in Rust?".to_string(),
        content: "I'm trying to write integration tests for a Warp router, but I'm not sure how to do it. Can someone help me?".to_string(),
        tags: Some(vec!["rust".to_string(), "warp".to_string()]),
    };

    // Test POST /questions to create a new question
    let post_resp = request()
        .method("POST")
        .path("/questions")
        .json(&question)
        .reply(&router.clone().routes())
        .await;

    assert_eq!(post_resp.status(), StatusCode::OK);

    // Test GET /questions to get the created question
    let get_resp = request()
        .method("GET")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .reply(&router.clone().routes())
        .await;

    assert_eq!(get_resp.status(), StatusCode::OK);

    // Deserialize the response body into a Value
    let response_body: Value =
        serde_json::from_slice(&get_resp.body()).expect("Failed to deserialize response body");
    let get_question: QuestionEntity = serde_json::from_value(response_body)
        .expect("Failed to deserialize response body into QuestionEntity");

    assert_eq!(get_question, question);

    // Test PUT /questions to update the created question
    let updated_question = QuestionEntity {
        id: question_id.clone(),
        title: "How to write integration tests in Rust?".to_string(),
        content: "I'm trying to write integration tests for a Warp router, but I'm not sure how to do it. Can someone help me?".to_string(),
        tags: Some(vec!["rust".to_string(), "warp".to_string(), "testing".to_string()]),
    };

    let put_resp = request()
        .method("PUT")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .json(&updated_question)
        .reply(&router.clone().routes())
        .await;

    assert_eq!(put_resp.status(), StatusCode::OK);

    // Test GET /questions to get the updated question
    let get_updated_resp = request()
        .method("GET")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .reply(&router.clone().routes())
        .await;

    assert_eq!(get_updated_resp.status(), StatusCode::OK);

    // Deserialize the response body into a Value
    let updated_response_body: Value = serde_json::from_slice(&get_updated_resp.body())
        .expect("Failed to deserialize response body");
    let get_updated_question: QuestionEntity = serde_json::from_value(updated_response_body)
        .expect("Failed to deserialize response body into QuestionEntity");

    assert_eq!(get_updated_question, updated_question);

    // Test DELETE /questions to delete the created question
    let delete_resp = request()
        .method("DELETE")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .reply(&router.clone().routes())
        .await;

    assert_eq!(delete_resp.status(), StatusCode::OK);

    // Test GET /questions to get the deleted question
    let get_deleted_resp = request()
        .method("GET")
        .path(format!("/questions/{}", &raw_question_id.clone()).as_str())
        .reply(&router.clone().routes())
        .await;

    assert_eq!(get_deleted_resp.status(), StatusCode::NOT_FOUND);
}
