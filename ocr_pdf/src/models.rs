use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct OcrResult {
    pub content: String,
}

impl OcrResult {
    pub fn new(content: String) -> Self {
        OcrResult {
            content: content.to_owned(),
        }
    }
}

impl fmt::Display for OcrResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OcrResult: {}", self.content)
    }
}