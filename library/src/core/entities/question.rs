use crate::core::entities::entity::Entity;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

/// Identifier for a question.
#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct QuestionId(pub(crate) String);

impl FromStr for QuestionId {
    type Err = Error;

    /// Attempts to parse a string into a `QuestionId`.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice to parse into a `QuestionId`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `QuestionId` or an `Error` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use library::core::entities::question::QuestionId;
    ///
    /// let id_str = "123";
    /// match QuestionId::from_str(id_str) {
    ///     Ok(id) => {
    ///         println!("Parsed question id: {:?}", id);
    ///     }
    ///     Err(err) => {
    ///         eprintln!("Failed to parse question id: {}", err);
    ///     }
    /// }
    /// ```
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

/// Represents a question entity.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestionEntity {
    /// Identifier for the question.
    pub id: QuestionId,
    /// Title of the question.
    pub title: String,
    /// Content of the question.
    pub content: String,
    /// Optional tags associated with the question.
    pub tags: Option<Vec<String>>,
}

impl QuestionEntity {
    /// Creates a new `QuestionEntity`.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifier for the question.
    /// * `title` - Title of the question.
    /// * `content` - Content of the question.
    /// * `tags` - Optional tags associated with the question.
    ///
    /// # Returns
    ///
    /// A new `QuestionEntity` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use library::core::entities::question::{QuestionEntity, QuestionId};
    ///
    /// let id = QuestionId::from_str("123").unwrap();
    /// let title = "Example Title".to_string();
    /// let content = "Example Content".to_string();
    /// let tags = Some(vec!["tag1".to_string(), "tag2".to_string()]);
    /// let question = QuestionEntity::new(id, title, content, tags);
    /// ```
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        QuestionEntity {
            id,
            title,
            content,
            tags,
        }
    }
}

impl Entity<QuestionEntity> for QuestionEntity {}
