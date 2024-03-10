use anyhow::Error;
use tonic::{
    async_trait,
    transport::{Channel, Endpoint},
};

use common::grpc::gpt_answer::gpt_answer::{
    gpt_answer_service_client::GptAnswerServiceClient, GetAnswerPayload,
};
use rust_core::{common::errors::CoreError, ports::gpt_answer::GptAnswerPort};

/// gRPC client for interacting with a GPT (Generative Pre-trained Transformer) answer service.
///
/// This struct represents a client for making gRPC calls to a GPT answer service. It provides
/// methods for connecting to the service, sending a question, and receiving an answer.
#[derive(Clone)]
pub struct GptAnswerClient {
    client: Option<GptAnswerServiceClient<Channel>>,
    endpoint: Endpoint,
}

impl GptAnswerClient {
    /// Creates a new `GptAnswerClient` instance with the provided gRPC endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint`: An `Endpoint` representing the gRPC communication endpoint.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `GptAnswerClient`.
    fn new(endpoint: Endpoint) -> Self {
        Self {
            client: None,
            endpoint,
        }
    }

    /// Initializes a new `GptAnswerClient` instance with the provided URI.
    ///
    /// # Arguments
    ///
    /// * `uri`: A `String` representing the URI of the GPT answer service.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the initialized `GptAnswerClient` if successful,
    /// or a `CoreError` if an error occurs during initialization.
    pub async fn init(uri: String) -> Result<Self, CoreError> {
        // Establish connection to the gRPC server
        let endpoint =
            Channel::from_shared(uri).map_err(|err| CoreError::InternalError(err.into()))?;

        Ok(Self::new(endpoint))
    }

    /// Establishes a connection to the GPT answer service at the specified URI.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the connected `GptAnswerServiceClient` if successful,
    /// or a `CoreError` if an error occurs during connection.
    pub async fn connect(&mut self) -> Result<(), CoreError> {
        let client = GptAnswerServiceClient::connect(self.endpoint.clone())
            .await
            .map_err(|err| CoreError::InternalError(err.into()))?;

        self.client = Some(client);
        Ok(())
    }
}

#[async_trait]
impl GptAnswerPort for GptAnswerClient {
    /// Sends a question to the GPT answer service and retrieves the generated answer.
    ///
    /// # Arguments
    ///
    /// * `question`: A `&str` representing the question to be sent to the service.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the generated answer as a `String` if successful,
    /// or a `CoreError` if an error occurs during communication with the service.
    async fn get_answer(&self, question: &str) -> Result<String, CoreError> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| CoreError::InternalError(Error::msg("Client not initialized")))?;

        let request = tonic::Request::new(GetAnswerPayload {
            question: question.to_string(),
        });

        let response = client
            .clone()
            .get_answer(request)
            .await
            .map_err(|err| CoreError::InternalError(err.into()))?;

        Ok(response.into_inner().answer)
    }
}
