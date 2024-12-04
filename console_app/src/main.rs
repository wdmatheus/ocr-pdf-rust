#[tokio::main]
async fn main() {
    let buffer = std::fs::read("/Users/wdmatheus/Desktop/oficio-requisitorio9.pdf")
        .expect("Failed to read file");

    match ocr_pdf::extract_text_from_buffer(&buffer).await {
        Ok(ocr) => println!("{}", ocr),
        Err(e) => println!("{}", e),
    }
}
