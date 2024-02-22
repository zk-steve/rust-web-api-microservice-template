use std::{str::FromStr, sync::Arc};

use library::{
    adapter::repositories::in_memory::question::QuestionInMemoryRepository,
    core::{
        entities::question::{QuestionEntity, QuestionId},
        ports::question::QuestionPort,
    },
    routes::Router,
};

#[test]
fn ready() {
    println!("Compiled successfully!")
}

#[tokio::test]
async fn test_router() {
    use warp::http::StatusCode;
    use warp::test::request;

    // Set up an in-memory question port for testing
    let question_port: Arc<dyn QuestionPort + Send + Sync> =
        Arc::new(QuestionInMemoryRepository::default());

    // Create a Router instance with the in-memory question port
    let router = Router::new(question_port.clone());

    // Test GET /questions
    let resp = request()
        .method("GET")
        .path("/questions")
        .reply(&router.clone().routes())
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    // Test a route that should return an error
    let resp = request()
        .method("GET")
        .path("/nonexistent")
        .reply(&router.clone().routes())
        .await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let question = QuestionEntity {
        id: QuestionId::from_str(String::from("question-1").as_str()).unwrap(), 
        title: "How to write integration tests in Rust?".to_string(),
        content: "I'm trying to write integration tests for a Warp router, but I'm not sure how to do it. Can someone help me?".to_string(),
        tags: Some(vec!["rust".to_string(), "warp".to_string()]),
    };

    let post_resp = request()
        .method("POST")
        .path("/questions")
        .json(&question)
        .reply(&router.clone().routes())
        .await;

    assert_eq!(post_resp.status(), StatusCode::OK);

    let created_question = question_port.get(&question.id.clone()).await.unwrap();

    assert_eq!(created_question, question);
}
