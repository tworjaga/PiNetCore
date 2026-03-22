use sqlx::{SqlitePool, Row};
use crate::config::Config;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Create tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS connections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ip TEXT NOT NULL,
                port INTEGER NOT NULL,
                device TEXT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn log_connection(&self, ip: &str, port: u16, device: Option<&str>) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO connections (ip, port, device) VALUES (?, ?, ?)",
        )
        .bind(ip)
        .bind(port as i64)
        .bind(device)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_recent_connections(&self, limit: i64) -> anyhow::Result<Vec<Connection>> {
        let rows = sqlx::query_as::<_, Connection>(
            "SELECT * FROM connections ORDER BY timestamp DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Connection {
    pub id: i32,
    pub ip: String,
    pub port: i32,
    pub device: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

