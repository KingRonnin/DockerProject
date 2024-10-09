use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database Error: {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("Internal Server Error")]
    InternalServerError,
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DbError(_) => HttpResponse::InternalServerError().body("Database error"),
            AppError::InternalServerError => HttpResponse::InternalServerError().body("Internal server error"),
        }
    }
}