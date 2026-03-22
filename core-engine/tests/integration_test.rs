#[tokio::test]
async fn test_packet_logging() {
    let db_url = ":memory:";
    let db = Database::new(db_url).await.unwrap();
    db.log_connection("192.168.1.1", 80, Some("test")).await.unwrap();
    let connections = db.get_recent_connections(10).await.unwrap();
    assert!(!connections.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;
}

