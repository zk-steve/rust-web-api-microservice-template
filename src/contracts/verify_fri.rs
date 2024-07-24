use std::str::FromStr;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::rest_client::aptos_api_types::{MoveType, VersionedEvent};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts::event_tracker::EventTracker;
use crate::contracts::helper::build_transaction;
use crate::contracts::types::Verify;

pub async fn verify_fri(config: &AppConfig, event_tracker: &mut EventTracker, data: Verify) -> anyhow::Result<VersionedEvent> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_statement").unwrap()),
            Identifier::new("verify_fri").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.proof,
                    data.fri_queue,
                    data.evaluation_point,
                    data.fri_step_size,
                    data.expected_root,
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let txd = config.client.submit(&tx).await?.into_inner().hash;
    println!("Verify FRI: {}", txd);


    let event = event_tracker.latest_event().await.unwrap();

    Ok(event)
}