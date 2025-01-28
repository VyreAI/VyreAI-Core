use std::env;
use std::error::Error;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub solana_rpc_url: String,
    pub ethereum_rpc_url: String,
    pub api_key: String,
    pub log_level: String,
}

impl Config {

    pub fn load_from_env() -> Result<Self, Box<dyn Error>> {
        let solana_rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let ethereum_rpc_url = env::var("ETHEREUM_RPC_URL")
            .unwrap_or_else(|_| "https://mainnet.infura.io/v3/your-project-id".to_string());
        let api_key = env::var("API_KEY")
            .unwrap_or_else(|_| "your-api-key".to_string());
        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            solana_rpc_url,
            ethereum_rpc_url,
            api_key,
            log_level,
        })
    }


    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !self.solana_rpc_url.starts_with("http") {
            return Err("Invalid Solana RPC URL.".into());
        }
        if !self.ethereum_rpc_url.starts_with("http") {
            return Err("Invalid Ethereum RPC URL.".into());
        }
        if self.api_key.is_empty() {
            return Err("API key is missing.".into());
        }
        if !["info", "debug", "warn", "error"].contains(&self.log_level.as_str()) {
            return Err("Invalid log level.".into());
        }
        Ok(())
    }
}


pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let config = Config::load_from_env()?;
    config.validate()?;
    Ok(config)
}
