use crate::repositories::postgres::models::question::QuestionModel;
use crate::repositories::postgres::schema::questions::dsl::questions;
use crate::repositories::postgres::schema::questions::id;
use async_trait::async_trait;
use rust_core::common::errors::Error;
use rust_core::entities::question::{QuestionEntity, QuestionId};
use rust_core::entities::question_filter::QuestionFilter;
use rust_core::ports::question::QuestionPort;

use deadpool_diesel::postgres::Pool;
use diesel::{delete, insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::{update, ExpressionMethods};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use super::models::id::ToId;

// NOTE: path relative to Cargo.toml
pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/repositories/postgres/migrations");

#[derive(Clone)]
pub struct QuestionDBRepository {
    pub db: Pool,
}

impl QuestionDBRepository {
    pub fn new(db: Pool) -> Self {
        QuestionDBRepository { db }
    }
}

#[async_trait]
impl QuestionPort for QuestionDBRepository {
    async fn add(&self, question: QuestionEntity) -> Result<QuestionEntity, Error> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let question = QuestionModel::from(question);
                let response = insert_into(questions)
                    .values(&question)
                    .get_result::<QuestionModel>(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => Error::NotFound,
                        _ => Error::InternalError,
                    });
                Ok(response.unwrap().to_entity())
            })
            .await
            .unwrap()
    }

    async fn update(&self, question: QuestionEntity) -> Result<QuestionEntity, Error> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let question = QuestionModel::from(question);
                let response = update(questions.filter(id.eq(question.id)))
                    .set(&question)
                    .get_result::<QuestionModel>(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => Error::NotFound,
                        _ => Error::InternalError,
                    })?;

                Ok(response.to_entity())
            })
            .await
            .unwrap()
    }

    async fn delete(&self, question_id: &QuestionId) -> Result<(), Error> {
        let question_id = question_id.to_id();
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let _ = delete(questions.filter(id.eq(question_id)))
                    .execute(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => Error::NotFound,
                        _ => Error::InternalError,
                    })?;

                Ok(())
            })
            .await
            .unwrap()
    }

    async fn get(&self, question_id: &QuestionId) -> Result<QuestionEntity, Error> {
        let question_id = question_id.to_id();
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = questions
                    .select(QuestionModel::as_select())
                    .find(question_id)
                    .first(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => Error::NotFound,
                        _ => Error::InternalError,
                    })?;

                Ok(response.to_entity())
            })
            .await
            .unwrap()
    }

    async fn list(&self, _question_filter: &QuestionFilter) -> Result<Vec<QuestionEntity>, Error> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let question_list = questions
                    .select(QuestionModel::as_select())
                    .load(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => Error::NotFound,
                        _ => Error::InternalError,
                    })?;

                Ok(question_list
                    .into_iter()
                    .map(|l| l.to_entity())
                    .collect::<Vec<_>>())
            })
            .await
            .unwrap()
    }
}
