use axum::{response::IntoResponse, http::StatusCode};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ColloError {
    #[error("Internal Server Error")]
    Database(#[from] sqlx::Error),

    #[error("Internal Server Error")]
    FileSystem(#[from] tokio::io::Error),

    #[error("Internal Server Error")]
    StringConversion(#[from] std::string::FromUtf8Error),
}

impl IntoResponse for ColloError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::FileSystem(_) | Self::StringConversion(_) | Self::Database(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}