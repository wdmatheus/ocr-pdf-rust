
pub async fn do_ocr(
    args: &Vec<String>,
    input_path: &String,
    output_path: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut cmd = tokio::process::Command::new("ocrmypdf");
    
    cmd.arg(input_path).arg(output_path).args(args);

    let output = cmd.output().await.expect("Failed to execute ocrmypdf");

    if !output.status.success() {
        return Err(format!(
            "Failed to run ocrmypdf with error: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
