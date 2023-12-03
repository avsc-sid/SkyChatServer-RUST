use axum::{response::{IntoResponse, Html}, http::StatusCode};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ColloError {
    #[error("Internal Server Error")]
    Database(#[from] sqlx::Error),

    #[error("Internal Server Error")]
    FileSystem(#[from] tokio::io::Error),

    #[error("Internal Server Error")]
    StringConversion(#[from] std::string::FromUtf8Error),

    #[error("Bad Request")]
    JsonError(#[from] serde_json::Error),
}

impl IntoResponse for ColloError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::FileSystem(_) | Self::StringConversion(_) | Self::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Html("server died.<br>good luck finding the error")).into_response()
            }

            Self::JsonError(_) => {
                StatusCode::BAD_REQUEST.into_response()
            } 
        }
    }
}
