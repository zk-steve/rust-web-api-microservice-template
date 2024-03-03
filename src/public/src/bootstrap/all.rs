use crate::bootstrap::router;

pub async fn run() {
    router::run().await;
}
