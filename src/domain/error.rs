use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> Self {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        Json::from(self).into_response()
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl Into<CommonError> for RepositoryError {
    fn into(self) -> CommonError {
        CommonError {
            message: self.message,
            code: 1,
        }
    }
}

impl From<sqlx::Error> for RepositoryError {
    fn from(value: sqlx::Error) -> Self {
        RepositoryError {
            message: value.to_string(),
        }
    }
}
