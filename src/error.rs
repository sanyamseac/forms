// src/error.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DbError(_) => HttpResponse::InternalServerError().json(self.to_string()),
            AppError::ValidationError(_) => HttpResponse::BadRequest().json(self.to_string()),
            AppError::NotFound(_) => HttpResponse::NotFound().json(self.to_string()),
            AppError::InternalError(_) => HttpResponse::InternalServerError().json(self.to_string()),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(self.to_string()),
        }
    }
}