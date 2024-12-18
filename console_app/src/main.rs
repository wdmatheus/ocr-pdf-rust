#[tokio::main]
async fn main() {
    let buffer = std::fs::read("/Users/wdmatheus/Desktop/oficio-requisitorio9.pdf")
        .expect("Failed to read file");

    let ocr_processor = ocr_pdf::ocr_processor::OcrProcessor;

    match ocr_processor.extract_text_from_buffer(&buffer).await {
        Ok(ocr) => println!("{}", String::from_utf8_lossy(&ocr)),
        Err(e) => println!("{}", e),
    }
}
