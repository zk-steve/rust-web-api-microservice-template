use aptos_sdk::move_types::value::MoveValue;

pub struct Verify {
    pub proof: MoveValue,
    pub fri_queue: MoveValue,
    pub evaluation_point: MoveValue,
    pub fri_step_size: MoveValue,
    pub expected_root: MoveValue,
}

pub struct InitFriGroup {
    pub fri_ctx: MoveValue,
}

pub struct ComputeNextLayer {
    pub channel_ptr: MoveValue,
    pub evaluation_point: MoveValue,
    pub fri_coset_size: MoveValue,
    pub fri_ctx: MoveValue,
    pub fri_queue_ptr: MoveValue,
    pub merkle_queue_ptr: MoveValue,
    pub n_queries: MoveValue,
}

pub struct VerifyMerkle {
    pub channel_ptr: MoveValue,
    pub merkle_queue_ptr: MoveValue,
    pub root: MoveValue,
    pub n_queries: MoveValue,
}