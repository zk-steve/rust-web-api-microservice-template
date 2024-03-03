#[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use openssl;

#[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use diesel;

use cli::bootstrap;

#[tokio::main]
async fn main() {
    bootstrap::all::run().await
}
