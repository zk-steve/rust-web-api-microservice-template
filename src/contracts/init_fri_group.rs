use aptos_sdk::crypto::HashValue;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use crate::config::AppConfig;
use crate::contracts::helper::{build_transaction};
use crate::contracts::types::InitFriGroup;

pub async fn init_fri_group(config: &AppConfig, data: InitFriGroup) -> anyhow::Result<HashValue> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("init_fri_group").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.fri_ctx
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let txd = config.client.submit(&tx).await?.into_inner().hash;
    println!("Init Fri Group {}", txd);
    Ok(HashValue::from(txd))
}