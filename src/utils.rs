use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum ApiError {
  #[error("bad request: {0}")]
  BadRequest(String),

  #[error("unauthorized")]
  Unauthorized,

  #[error("not found")]
  NotFound,

  #[error("internal server error")]
  Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ApiErrorResponse {
  error: String,
  code: u16,
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let (status, msg) = match &self {
      ApiError::BadRequest(s) => (StatusCode::BAD_REQUEST, s.clone()),
      ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
      ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
      ApiError::Internal(e) => {
        error!("internal error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".into())
      }
    };

    let body = Json(ApiErrorResponse {
      error: msg,
      code: status.as_u16(),
    });

    (status, body).into_response()
  }
}