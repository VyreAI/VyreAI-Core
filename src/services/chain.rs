use solana_sdk::{signature::{Keypair, Signer}, transaction::Transaction};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use async_trait::async_trait;
use crate::api::models::{AnalyzeRequest, AnalyzeResponse};

const SOLANA_RPC_URL: &str = "https://api.mainnet-beta.solana.com";


#[derive(Serialize, Deserialize, Debug)]
pub struct SolanaAccountInfo {
    pub balance: f64,
    pub lamports: u64,
}

pub struct SolanaService {
    client: RpcClient,
}

impl SolanaService {
    pub fn new(rpc_url: &str) -> Self {
        let client = RpcClient::new(rpc_url.to_string());
        SolanaService { client }
    }

    pub fn get_account_balance(&self, pubkey: &str) -> Result<SolanaAccountInfo, String> {
        let pubkey = Pubkey::from_str(pubkey)
            .map_err(|e| format!("Invalid public key: {}", e))?;

        let balance = self.client.get_balance(&pubkey)
            .map_err(|e| format!("Error fetching balance: {}", e))?;

        info!("Account balance for {}: {} lamports", pubkey, balance);
        
        Ok(SolanaAccountInfo {
            balance: balance as f64 / 1_000_000_000.0,
            lamports: balance,
        })
    }


    pub fn get_transaction_details(&self, tx_signature: &str) -> Result<String, String> {
        let tx = self.client.get_transaction(tx_signature)
            .map_err(|e| format!("Error fetching transaction: {}", e))?;

        info!("Transaction details: {:?}", tx);
        
        Ok(format!("{:?}", tx))
    }

    pub fn check_account_exists(&self, pubkey: &str) -> Result<bool, String> {
        let pubkey = Pubkey::from_str(pubkey)
            .map_err(|e| format!("Invalid public key: {}", e))?;

        let account = self.client.get_account(&pubkey)
            .map_err(|e| format!("Error fetching account: {}", e))?;

        Ok(account.is_some())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockchainAnalysisRequest {
    pub chain: String,
    pub transaction_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockchainAnalysisResponse {
    pub result: String,
    pub details: String,
}

#[async_trait]
pub trait BlockchainAnalyzer {
    async fn analyze(&self, request: AnalyzeRequest) -> Result<AnalyzeResponse, Box<dyn Error>>;
}

#[async_trait]
impl BlockchainAnalyzer for SolanaService {
    async fn analyze(&self, request: AnalyzeRequest) -> Result<AnalyzeResponse, Box<dyn Error>> {
        let analysis_result = match request.chain.to_lowercase().as_str() {
            "solana" => {
                let blockhash = self.fetch_latest_blockhash().await?;
                let impact = self.analyze_transaction_impact(&request.data).await?;
                format!(
                    "Solana Blockhash: {}, Impact: {}",
                    blockhash, impact
                )
            }
            "ethereum" => {
                "Ethereum blockchain analysis is under construction.".to_string()
            }
            _ => "Unsupported chain.".to_string(),
        };

        Ok(AnalyzeResponse {
            result: analysis_result,
        })
    }
}

pub async fn analyze_large_transactions() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut whale_map = HashMap::new();
    let transactions = vec![
        ("0xWhale1", "1000 SOL transferred"),
        ("0xWhale2", "500 ETH transferred"),
        ("0xWhale3", "2500 SOL transferred"),
    ];

    for (address, transaction) in transactions {
        let impact = if transaction.contains("SOL") {
            "Market-moving transaction"
        } else {
            "Non-market impacting"
        };
        whale_map.insert(address.to_string(), impact.to_string());
    }

    Ok(whale_map)
}
