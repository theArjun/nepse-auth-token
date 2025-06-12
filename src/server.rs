use axum::{http::StatusCode, response::Json, routing::get, Router};
use crate::models::{ErrorResponse, ParsedTokenResult};
use crate::nepse::get_access_token;

pub async fn get_access_token_handler()
-> Result<Json<ParsedTokenResult>, (StatusCode, Json<ErrorResponse>)> {
    match get_access_token().await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

pub fn create_router() -> Router {
    Router::new().route("/", get(get_access_token_handler))
}

pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router();
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    println!("🚀 Server running on http://127.0.0.1:{}", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
} 