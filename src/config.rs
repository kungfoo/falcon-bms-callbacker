use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_level: String,
    pub listen_address: String,
    pub listen_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config { log_level: "info".to_string(), listen_address: "0.0.0.0".to_string(), listen_port: 9027 }
    }
}