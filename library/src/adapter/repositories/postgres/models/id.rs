use crate::core::entities::question::QuestionId;

impl QuestionId {
    pub fn to_id(&self) -> i32 {
        self.0.parse().unwrap()
    }
}
