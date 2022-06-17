use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Enum for the different error types.
pub enum ApiError {
    SomethingWentWrong,
    ResourceNotFound,
}

// Allows handlers to return an ApiError directly, which will be mapped to the correct HTTP status code.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::SomethingWentWrong => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ResourceNotFound => StatusCode::NOT_FOUND,
        };

        (status).into_response()
    }
}

// Allows SQLx errors to be converted to ApiErrors.
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::ResourceNotFound,
            _ => ApiError::SomethingWentWrong,
        }
    }
}
