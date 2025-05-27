use crate::errors::ServiceError;
use std::sync::OnceLock;

pub struct Config {
    pub database_url: String,
    pub rocksdb_path: String,
    pub rocksdb_buffer_size: usize,
    pub gmail_username: String,
    pub gmail_app_password: String,
    pub jwt_secret: String,
    pub architector_server_url: String,
    pub test_vultr_api_key: String,
}

impl Config {
    fn new() -> Result<Config, ServiceError> {
        dotenv::dotenv().ok();
        Ok(Config {
            database_url: std::env::var("DATABASE_URL").unwrap(),
            rocksdb_path: std::env::var("ROCKSDB_PATH")
                .unwrap_or_else(|_| "./rocksdb_data".to_string()),
            rocksdb_buffer_size: std::env::var("ROCKSDB_BUFFER_SIZE")
                .unwrap_or_else(|_| (1024 * 1024).to_string())
                .parse()
                .unwrap(),
            gmail_username: std::env::var("GMAIL_USERNAME").unwrap(),
            gmail_app_password: std::env::var("GMAIL_APP_PASSWORD").unwrap(),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string()),
            architector_server_url: std::env::var("ARCHITECTOR_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
            test_vultr_api_key: std::env::var("TEST_VULTR_API_KEY").unwrap_or_else(|_| "test_api_key".to_string()),
        })
    }
}
pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| Config::new().unwrap())
}
