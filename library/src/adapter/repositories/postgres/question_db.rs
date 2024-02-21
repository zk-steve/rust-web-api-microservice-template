use crate::adapter::repositories::postgres::models::question::QuestionModel;
use crate::adapter::repositories::postgres::schema::questions::dsl::questions;
use crate::adapter::repositories::postgres::schema::questions::id;
use crate::common::errors::Error;
use crate::core::entities::question::{QuestionEntity, QuestionId};
use async_trait::async_trait;
use deadpool_diesel::postgres::Pool;

use diesel::{delete, insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::{update, ExpressionMethods};

use crate::core::entities::question_filter::QuestionFilter;
use crate::core::ports::question::QuestionPort;

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
                let updated: QuestionModel = insert_into(questions)
                    .values(&question)
                    .get_result(conn)
                    .unwrap();
                Ok(updated.to_entity())
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
                let updated: QuestionModel = update(questions.filter(id.eq(question.id)))
                    .set(&question)
                    .get_result(conn)
                    .unwrap();
                Ok(updated.to_entity())
            })
            .await
            .unwrap()
    }

    async fn delete(&self, _question_id: &QuestionId) -> Result<(), Error> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                delete(questions.filter(id.eq(1))).execute(conn).unwrap();
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
                let model = questions
                    .select(QuestionModel::as_select())
                    .find(question_id)
                    .first(conn)
                    .unwrap();
                Ok(model.to_entity())
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
                Ok(questions
                    .select(QuestionModel::as_select())
                    .load(conn)
                    .unwrap()
                    .iter()
                    .map(|l| l.to_entity())
                    .collect::<Vec<_>>())
            })
            .await
            .unwrap()
    }
}
