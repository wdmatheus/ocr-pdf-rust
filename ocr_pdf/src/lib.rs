use ocrmypdf_rs::{Ocr, OcrMyPdf};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct OcrResult {
    pub content: String,
}

impl OcrResult {
    pub fn new(content: &str) -> Self {
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

pub async fn extract_text_from_buffer(buffer: &[u8]) -> Result<OcrResult, Box<dyn std::error::Error>> {

    let dirname = "./dest-files";

    match tokio::fs::create_dir_all(dirname).await
    {
        Ok(_) => {},
        Err(e) => return Err(format!("Error creating directory: {}", e).into()),
    };

    let prefix = format!("./{}/{}", dirname, Uuid::new_v4().to_string());

    let pdf_path = format!("./{}.pdf", prefix);

    let save_pdf = tokio::fs::write(&pdf_path, buffer).await;

    if save_pdf.is_err() {
        return Err(format!("Error saving PDF: {}", save_pdf.err().unwrap()).into());
    }

    extract_text_from_pdf(&pdf_path, &prefix).await
}

pub async  fn extract_text_from_pdf(input_path: &str, prefix: &str) -> Result<OcrResult, Box<dyn std::error::Error>> {

    let output_pdf = format!("./{}.pdf", &prefix);
    let output_txt = format!("./{}.txt", &prefix);

    let args: Vec<String> = vec![
        "-l".into(),
        "por".into(),
        "--quiet".into(),
        "--optimize=0".into(),
        "--tesseract-timeout=120".into(),
        "--force-ocr".into(),
        "--jobs=4".into(),
        "--output-type=pdf".into(),
        "--pdf-renderer=sandwich".into(),
        "--sidecar".into(),
        output_txt.clone(),
    ];

    let mut ocr = OcrMyPdf::new(
        Some(args),
        Some(input_path.to_owned()),
        Some(output_pdf.to_owned()));

    ocr.execute();          

    match tokio::fs::read_to_string(&output_txt).await {
        Ok(content) => {
            remove_temp_files(vec![&output_pdf, &output_txt]).await;
            Ok(OcrResult::new(&content))
        },
        Err(e) => Err(format!("Error reading OCR output: {}", e).into()),
    }
}

async fn remove_temp_files(paths: Vec<&str>) {
    for path in paths{
        tokio::fs::remove_file(path).await.unwrap_or(());
    }
}
