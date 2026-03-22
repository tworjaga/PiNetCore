// Common test helpers
pub mod test_db;

use crate::storage::Database;
use tokio::test;

pub async fn setup_db() -> Database {
    Database::new(":memory:").await.unwrap()
}

