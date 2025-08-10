use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
    pub database_url: String,
    pub database_max_connections: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_host: "127.0.0.1".to_string(),
            server_port: 8080,
            log_level: "info".to_string(),
            database_url: "postgresql://postgres:password@localhost:5432/rustmicro".to_string(),
            database_max_connections: 10,
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            log_level: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/rustmicro".to_string()),
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        }
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    pub fn server_url(&self) -> String {
        format!("http://{}:{}", self.server_host, self.server_port)
    }
}
