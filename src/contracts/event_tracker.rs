use std::time::Duration;
use aptos_sdk::rest_client::aptos_api_types::{MoveType, VersionedEvent};
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::account_address::AccountAddress;

pub struct EventTracker {
    client: Client,
    account_address: AccountAddress,
    creation_number: u64,
    typ: MoveType,
}

impl EventTracker {
    pub fn new(
        client: Client,
        account_address: AccountAddress,
        typ: MoveType,
        creation_number: u64,
    ) -> Self {
        Self {
            client,
            account_address,
            typ,
            creation_number,
        }
    }
}
impl EventTracker {

    pub async fn latest_event(&mut self) -> Option<VersionedEvent> {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let mut result = None;
        loop {
            let creation_number = self.creation_number + 1;
            let events = self.client.get_account_events_by_creation_number(
                self.account_address,
                creation_number,
                None,
                None,
            ).await.unwrap().into_inner();
            if events.len() == 0 { break; };
            self.creation_number = creation_number;
            events.into_iter().for_each(|e| {
               if e.typ == self.typ {
                    result = Some(e);
                }
            })
        }
        result
    }
}