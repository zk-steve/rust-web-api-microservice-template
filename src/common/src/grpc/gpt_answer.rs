/// Module for gRPC service definitions related to answering questions with GPT (Generative Pre-trained Transformer) models.
///
/// This module includes generated gRPC service definitions for answering questions using GPT models.
/// The `tonic::include_proto!` macro is used to include the protobuf definitions, enabling easy
/// integration of gRPC services into Rust code.
pub mod gpt_answer {
    // Include the protobuf definitions for the gpt_answer service.
    tonic::include_proto!("gpt_answer");
}
