use uuid::Uuid;
use crate::models::OcrResult;
use crate::ocr_my_pdf;

#[derive(Default)]
pub struct OcrProcessor;

impl OcrProcessor {
    pub async fn extract_text_from_buffer(
        &self,
        buffer: &[u8],
    ) -> Result<OcrResult, Box<dyn std::error::Error>> {
        let dirname = "./dest-files";

        match tokio::fs::create_dir_all(dirname).await {
            Ok(_) => {}
            Err(e) => return Err(format!("Error creating directory: {}", e).into()),
        };

        let prefix = format!("./{}/{}", dirname, Uuid::new_v4());

        let pdf_path = format!("./{}.pdf", prefix);

        let save_pdf = tokio::fs::write(&pdf_path, buffer).await;

        if save_pdf.is_err() {
            return Err(format!("Error saving PDF: {}", save_pdf.err().unwrap()).into());
        }

        self.extract_text_from_pdf(&pdf_path, &prefix).await
    }

    pub async fn extract_text_from_pdf(
        &self,
        input_path: &str,
        prefix: &str,
    ) -> Result<OcrResult, Box<dyn std::error::Error>> {
        let output_pdf = format!("./{}.pdf", &prefix);
        let output_txt = format!("./{}.txt", &prefix);

        let args: Vec<String> = vec![
            "-l".into(),
            "por".into(),
            "--quiet".into(),
            "--optimize=0".into(),
            "--tesseract-timeout=60".into(),
            "--force-ocr".into(),
            "--jobs=1".into(),
            "--output-type=pdf".into(),
            "--pdf-renderer=sandwich".into(),
            "--sidecar".into(),
            output_txt.clone(),
        ];

        match  ocr_my_pdf::do_ocr(&args, &input_path.to_owned(),&output_pdf).await{
            Ok(_) => {}
            Err(e) => return Err(format!("{}", e).into()),
        };

        match tokio::fs::read_to_string(&output_txt).await {
            Ok(content) => {
                self.remove_temp_files(vec![input_path, &output_pdf, &output_txt])
                    .await;
                Ok(OcrResult::new(content))
            }
            Err(e) => Err(format!("Error reading OCR output: {}", e).into()),
        }
    }

    async fn remove_temp_files(&self, paths: Vec<&str>) {
        for path in paths {
            tokio::fs::remove_file(path).await.unwrap_or(());
        }
    }
}