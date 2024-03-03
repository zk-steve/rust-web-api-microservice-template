use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DBConfig {
    pub url: String,
    pub max_size: usize,
}
