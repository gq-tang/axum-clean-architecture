use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub message: String,
    pub code: u32,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success() -> Self {
        ApiResponse {
            message: "success".to_string(),
            code: 0,
            data: None,
        }
    }
    pub fn new(data: T) -> Self {
        ApiResponse {
            message: "success".to_string(),
            code: 0,
            data: Some(data),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
