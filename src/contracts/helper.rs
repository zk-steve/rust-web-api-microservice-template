use std::str::FromStr;
use std::time::{SystemTime};
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::LocalAccount;
use aptos_sdk::types::transaction::{SignedTransaction, TransactionPayload};

pub fn str_to_u256(s: &str) -> MoveValue {
    let u256_value = U256::from_str(s).unwrap();
    MoveValue::U256(u256_value)
}

pub fn str_to_bool(s: &str) -> bool {
    let bool_str = s.trim_start_matches("Bool(").trim_end_matches(")");
    bool::from_str(bool_str).unwrap()
}

pub fn build_transaction(payload: TransactionPayload, sender: &LocalAccount, chain_id: ChainId) -> SignedTransaction {
    let i = sender.increment_sequence_number();
    let tx = TransactionBuilder::new(
        payload,
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 60,
        chain_id,
    )
        .sender(sender.address())
        .sequence_number(i)
        .max_gas_amount(10000)
        .gas_unit_price(100)
        .build();
    sender.sign_transaction(tx)
}
