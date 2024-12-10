use axum::{
    body::Body,
    extract::Multipart,
    http::{Request, Uri},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use chrono::prelude::Utc as chrono_utc;
use ocr_pdf::ocr_processor::OcrProcessor;
use serde::Serialize;
use std::fmt;
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

#[tokio::main]
async fn main() {
    //let state = Arc::new(OcrProcessor);

    let app = Router::new()
        .route("/ocr-pdf", post(extract_text_from_pdf))
        .layer(axum::middleware::from_fn(uri_middleware))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<Body>, _span: &Span| {
                    println!(
                        "[{}] | received request at path: {}",
                        chrono_utc::now(),
                        request.uri().path()
                    )
                })
                .on_response(|response: &Response, _latency: Duration, _span: &Span| {
                    println!(
                        "[{}] | path: {} | elapsed {} ms",
                        chrono_utc::now(),
                        response
                            .extensions()
                            .get::<RequestUri>()
                            .map(|r| &r.0)
                            .unwrap_or(&Uri::default()),
                        _latency.as_millis()
                    )
                })
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        )
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn extract_text_from_pdf(
    mut multipart: Multipart
) -> impl IntoResponse {
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

    //Ok(Json(ocr_pdf::models::OcrResult::new(chrono_utc::now().to_string())))    

    match OcrProcessor.extract_text_from_buffer(&buffer).await {
        Ok(ocr) => Ok(Json(ocr)),
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

#[derive(Clone)]
struct RequestUri(Uri);

impl fmt::Display for RequestUri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OcrResult: {}", self.0.path())
    }
}

async fn uri_middleware(request: Request<Body>, next: Next) -> Response {
    let uri = request.uri().clone();

    let mut response = next.run(request).await;

    response.extensions_mut().insert(RequestUri(uri));

    response
}
