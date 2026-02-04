use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub index: IndexConfig,
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: SocketAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    pub data_dir: String,
    pub index_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_seconds: u64,
    pub max_size: usize,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn from_env() -> anyhow::Result<Self> {
        let server_address = std::env::var("SERVER_ADDRESS")
            .unwrap_or_else(|_| "0.0.0.0:50051".to_string());
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let data_dir = std::env::var("DATA_DIR")
            .unwrap_or_else(|_| "./data".to_string());
        let index_path = std::env::var("INDEX_PATH")
            .unwrap_or_else(|_| "./index".to_string());

        Ok(Config {
            server: ServerConfig {
                address: server_address.parse()?,
            },
            redis: RedisConfig {
                url: redis_url,
                pool_size: 10,
            },
            index: IndexConfig {
                data_dir,
                index_path,
            },
            cache: CacheConfig {
                enabled: true,
                ttl_seconds: 3600,
                max_size: 10000,
            },
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                address: "0.0.0.0:50051".parse().unwrap(),
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
            },
            index: IndexConfig {
                data_dir: "./data".to_string(),
                index_path: "./index".to_string(),
            },
            cache: CacheConfig {
                enabled: true,
                ttl_seconds: 3600,
                max_size: 10000,
            },
        }
    }
}
