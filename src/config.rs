use std::str::FromStr;
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::crypto::ValidCryptoMaterialStringExt;
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::{AccountKey, LocalAccount};
use dotenv::dotenv;
use color_eyre::Result;
pub struct EnvConfig {
    pub node_url: String,
    pub private_key: String,
    pub account_address: String,
    pub module_address: String,
    pub chain_id: String,
    pub creation_number: String,
}

pub fn get_env_var(key: &str) -> Result<String> {
    std::env::var(key).map_err(|e| e.into())
}

pub fn get_env_var_or_panic(key: &str) -> String {
    get_env_var(key).unwrap_or_else(|e| panic!("Failed to get env var {}: {}", key, e))
}

impl EnvConfig {
    pub fn new() -> Self {
        dotenv().ok().expect("Failed to load .env file");
        let node_url = get_env_var_or_panic("APTOS_NODE_URL");
        let private_key = get_env_var_or_panic("APTOS_PRIVATE_KEY");
        let account_address = get_env_var_or_panic("APTOS_ACCOUNT_ADDRESS");
        let module_address = get_env_var_or_panic("APTOS_MODULE_ADDRESS");
        let chain_id = get_env_var_or_panic("CHAIN_ID");
        let creation_number = get_env_var_or_panic("CREATION_NUMBER");
        EnvConfig {
            chain_id,
            node_url,
            private_key,
            account_address,
            module_address,
            creation_number,
        }
    }
}

pub struct AppConfig {
    pub client: Client,
    pub account: LocalAccount,
    pub module_address: AccountAddress,
    pub chain_id: ChainId,
    pub creation_number: u64,
}

impl From<EnvConfig> for AppConfig {
    fn from(value: EnvConfig) -> Self {
        let client = Client::new(value.node_url.parse().unwrap());
        let private_key = Ed25519PrivateKey::from_encoded_string(&value.private_key).expect("Failed to parse private key");
        let account_key = AccountKey::from(private_key);

        let account_address = value.account_address.parse().expect("Invalid account address");
        let account = LocalAccount::new(account_address, account_key, 500);


        let module_address = value.module_address.parse().expect("Invalid module address");
        let chain_id = ChainId::from_str(&value.chain_id).expect("Invalid chain id");
        AppConfig {
            client,
            account,
            module_address,
            chain_id,
            creation_number: value.creation_number.parse().unwrap(),
        }
    }
}

