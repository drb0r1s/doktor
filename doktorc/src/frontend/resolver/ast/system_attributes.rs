use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemAttributes {
    // Image
    pub source: Option<String>,
    // Text
    pub content: Option<String>,
    // Input
    pub placeholder: Option<String>,
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
}