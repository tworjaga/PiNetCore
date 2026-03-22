use super::*;
use crate::storage::Database;

#[tokio::test]
async fn test_db_insert() {
    let db = setup_db().await;
    db.log_connection("test", 80, None).await.unwrap();
}

