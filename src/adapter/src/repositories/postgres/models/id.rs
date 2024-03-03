use rust_core::entities::question::QuestionId;

pub trait ToId {
    fn to_id(&self) -> i32;
}

// Implement the trait for the foreign type
impl ToId for QuestionId {
    fn to_id(&self) -> i32 {
        self.0.parse().unwrap()
    }
}
