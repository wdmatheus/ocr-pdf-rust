use axum::http::StatusCode;
use axum::{
    extract::Multipart,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use ocr_pdf::ocr_processor::OcrProcessor;
use serde::Serialize;
use tower::{ServiceBuilder};
use tower_http::{compression::CompressionLayer};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ocr-pdf", post(extract_text_from_pdf))
        .layer(
            ServiceBuilder::new()                
                .layer(CompressionLayer::new())
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn extract_text_from_pdf(mut multipart: Multipart) -> impl IntoResponse {
    let mut buffer: Vec<u8> = Vec::new();
    let mut total: u8 = 0;

    while let Some(field) = multipart.next_field().await.unwrap() {
        total += 1;

        if total > 1 {
            return Err(ErrorResponse::new("Only one file is allowed").into_response());
        }

        match field.bytes().await {
            Ok(bytes) => buffer.extend_from_slice(&bytes),
            Err(e) => {
                return Err(
                    ErrorResponse::new(&format!("Error reading file: {}", e)).into_response()
                )
            }
        };
    }

    match OcrProcessor.extract_text_from_buffer(&buffer).await {
        Ok(ocr) => Ok((StatusCode::OK, [("content-type", "text/plain")], ocr)),
        Err(e) => Err(ErrorResponse::new(&e).into_response()),
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        ErrorResponse {
            message: message.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (axum::http::StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}
