use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Configuration for database connection.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
}

impl DatabaseConfig {
    /// Creates a new DatabaseConfig from environment variables.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            host: std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("POSTGRES_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .unwrap_or(5432),
            database: std::env::var("POSTGRES_DATABASE")?,
            username: std::env::var("POSTGRES_USER")?,
            password: std::env::var("POSTGRES_PASSWORD")?,
            max_connections: std::env::var("POSTGRES_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            min_connections: std::env::var("POSTGRES_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),
            connect_timeout_secs: std::env::var("POSTGRES_CONNECT_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        })
    }

    /// Builds the connection URL for PostgreSQL.
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    /// Creates a connection pool using this configuration.
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(Duration::from_secs(self.connect_timeout_secs))
            .connect(&self.connection_url())
            .await
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "jira".to_string(),
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout_secs: 30,
        }
    }
}
