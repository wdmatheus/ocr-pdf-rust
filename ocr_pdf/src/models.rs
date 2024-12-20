
#[derive(Debug, serde::Serialize)]
pub struct OcrResult {
    pub content: String,
}

impl OcrResult {
    pub fn new(content: String) -> Self {
        OcrResult { content }
    }
}