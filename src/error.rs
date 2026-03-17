use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Internal(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Rsa(#[from] rsa::errors::Error),
    #[error(transparent)]
    Pkcs8(#[from] rsa::pkcs8::Error),
    #[error(transparent)]
    Spki(#[from] rsa::pkcs8::spki::Error),
    #[error(transparent)]
    AddrParse(#[from] std::net::AddrParseError),
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Internal(_)
            | AppError::Io(_)
            | AppError::Jwt(_)
            | AppError::Rsa(_)
            | AppError::Pkcs8(_)
            | AppError::Spki(_)
            | AppError::AddrParse(_)
            | AppError::UrlParse(_)
            | AppError::Yaml(_)
            | AppError::Http(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = axum::Json(ErrorBody {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
