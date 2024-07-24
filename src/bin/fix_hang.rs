use std::str::FromStr;

use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;
use aptos_sdk::rest_client::aptos_api_types::{MoveType, VersionedEvent};

use verifier_onchain_services::config::{AppConfig, EnvConfig};
use verifier_onchain_services::contracts::event_tracker::EventTracker;
use verifier_onchain_services::contracts::helper::str_to_u256;
use verifier_onchain_services::contracts::types::VerifyMerkle;
use verifier_onchain_services::contracts::verify_merkle::{verify_merkle, verify_merkle_view};

#[tokio::main]
async fn main() {
    let config = AppConfig::from(EnvConfig::new());
    let sequence_number = config.client.get_account(config.account.address()).await.unwrap().into_inner().sequence_number;
    eprintln!("sequence_number = {:#?}", sequence_number);
    config.account.set_sequence_number(sequence_number);

    let mut n_queries: VersionedEvent;

    let name = format!("{}::fri_layer::NQueries", config.module_address);
    let mut n_queries_event_tracker = EventTracker::new(
        config.client.clone(),
        config.account.address(),
        MoveType::from_str(&name).unwrap(),
        config.creation_number,
    );

    n_queries = n_queries_event_tracker.latest_event().await.unwrap();

    let mut compute_next_layer_event_tracker = EventTracker::new(
        config.client.clone(),
        config.account.address(),
        MoveType::from_str(&format!("{}::fri_statement::ComputeNextLayer", config.module_address)).unwrap(),
        config.creation_number,
    );
    let event_compute = compute_next_layer_event_tracker.latest_event().await.unwrap();

    let input_verify_merkle = VerifyMerkle {
        channel_ptr: str_to_u256(event_compute.data.get("channel_ptr").unwrap().as_str().unwrap()),
        merkle_queue_ptr: str_to_u256(event_compute.data.get("merkle_queue_ptr").unwrap().as_str().unwrap()),
        root: MoveValue::U256(U256::from_str("9390404794146759926609078012164974184924937654759657766410025620812402262016").unwrap()),
        n_queries: str_to_u256(n_queries.data.get("n_queries").unwrap().as_str().unwrap()),
    };

    loop {
        verify_merkle(&config, &input_verify_merkle).await.expect("E");
        if !verify_merkle_view(&config).await.unwrap() {
            break;
        }
        println!("merkle_verifier {}", true);
    }

    eprintln!("done");
}