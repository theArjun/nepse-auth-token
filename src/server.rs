use axum::{http::StatusCode, response::Json, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::{info, error, debug};
use crate::models::{ErrorResponse, ParsedTokenResult};
use crate::nepse::get_access_token;

pub async fn get_access_token_handler()
-> Result<Json<ParsedTokenResult>, (StatusCode, Json<ErrorResponse>)> {
    debug!("🔑 Processing access token request");
    
    match get_access_token().await {
        Ok(result) => {
            debug!("✅ Successfully generated access token");
            Ok(Json(result))
        },
        Err(e) => {
            error!("❌ Failed to generate access token: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            ))
        }
    }
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_access_token_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|_request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::debug!("📥 Incoming request");
                })
                .on_response(|_response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!("📤 Response sent in {:.2}ms", latency.as_secs_f64() * 1000.0);
                })
        )
}

pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router();
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    info!("🚀 Server running on http://127.0.0.1:{}", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
} 