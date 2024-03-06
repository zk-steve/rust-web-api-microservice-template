#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::{str::FromStr, sync::Arc};

    use deadpool_diesel::{
        postgres::{Pool, Runtime},
        Manager,
    };
    use diesel_migrations::MigrationHarness;
    use testcontainers_modules::{postgres::Postgres, testcontainers::clients::Cli};

    use rust_core::{
        entities::{
            pagination_entity::PaginationEntity,
            question::{QuestionEntity, QuestionId},
            question_filter::QuestionFilter,
        },
        ports::question::QuestionPort,
    };

    use crate::repositories::in_memory::question::QuestionInMemoryRepository;
    use crate::repositories::postgres::question_db::{QuestionDBRepository, MIGRATIONS};

    struct DatabaseConfig {
        url: String,
        max_size: usize,
    }

    async fn test_question_repository<T>(question_port: Arc<T>)
    where
        T: QuestionPort + Send + Sync,
    {
        let raw_question_id = "1";
        let question_id = QuestionId::from_str(raw_question_id).unwrap();

        let question = QuestionEntity {
            id: question_id.clone(),
            title: "How to write integration tests in Rust?".to_string(),
            content: "I'm trying to write integration tests for a Warp router, but I'm not sure how to do it. Can someone help me?".to_string(),
            tags: Some(vec!["rust".to_string(), "warp".to_string()]),
        };

        let result = question_port.add(question.clone()).await;
        assert_eq!(result.unwrap(), question);

        let result = question_port.get(&question_id).await;
        assert_eq!(result.unwrap(), question);

        let mut query_params = HashMap::new();
        query_params.insert("start".to_string(), "0".to_string());
        query_params.insert("end".to_string(), "10".to_string());

        let pagination = match PaginationEntity::from_query(&query_params) {
            Ok(pagination_entity) => pagination_entity,
            Err(err) => {
                panic!("Failed to parse pagination entity: {:?}", err);
            }
        };
        let question_filter = QuestionFilter { pagination };
        let result = question_port.list(&question_filter).await;
        assert_eq!(result.unwrap().len(), 1);

        let updated_question = QuestionEntity {
            id: question_id.clone(),
            title: "Rust in action".to_string(),
            content: "Rust is a language".to_string(),
            tags: Some(vec![
                "rust".to_string(),
                "warp".to_string(),
                "test".to_string(),
            ]),
        };

        let result = question_port.update(updated_question.clone()).await;
        assert_eq!(result.unwrap(), updated_question);

        let result = question_port.delete(&question_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn question_in_memory_repository_test() {
        let question_port: Arc<QuestionInMemoryRepository> =
            Arc::new(QuestionInMemoryRepository::default());
        test_question_repository(question_port).await;
    }

    #[tokio::test]
    async fn question_postgres_repository_test() {
        // Set up a postgres database question port for testing
        let docker = Cli::default();
        let postgres_instance = docker.run(Postgres::default());

        let database_url = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres_instance.get_host_port_ipv4(5432)
        );
        let database_config = DatabaseConfig {
            url: database_url.clone(),
            max_size: 10,
        };

        let manager = Manager::new(database_config.url, Runtime::Tokio1);
        let db_pool = Pool::builder(manager)
            .max_size(database_config.max_size)
            .build()
            .unwrap();

        let conn = db_pool.get().await.unwrap();

        // Migration the database
        let result = conn
            .interact(|connection| {
                let result = MigrationHarness::run_pending_migrations(connection, MIGRATIONS);
                match result {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            })
            .await;
        assert!(result.is_ok(), "Failed to run the migrations");

        let question_port = Arc::new(QuestionDBRepository::new(db_pool.clone()));

        test_question_repository(question_port).await;
    }
}
