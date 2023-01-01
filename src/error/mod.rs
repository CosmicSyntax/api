use std::error::Error;
use std::fmt::Display;

use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiErrors {
    ServerStartError(&'static str),
    DBError(&'static str),
    BadRequest(&'static str),
}

pub const SERVER_START_ERROR: ApiErrors = ApiErrors::ServerStartError("could not start server");
pub const BAD_REQUEST_ERROR: ApiErrors = ApiErrors::BadRequest("request is not correct");
pub const DB_ERROR: ApiErrors = ApiErrors::DBError("db constraint issues");
pub const DB_ERROR_USER_EXISTS: ApiErrors = ApiErrors::DBError("Username already exists");

impl Error for ApiErrors {}

impl ResponseError for ApiErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiErrors::ServerStartError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiErrors::DBError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut res = HttpResponse::build(self.status_code());
        res.insert_header(ContentType::json());

        res.body(BoxBody::new(
            json!({
                "error": self.status_code().to_string(),
                "message": self.to_string(),
            })
            .to_string(),
        ))
    }
}

impl Display for ApiErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiErrors::ServerStartError(msg) => {
                write!(f, "{}", msg)
            }
            ApiErrors::DBError(msg) => {
                write!(f, "{}", msg)
            }
            ApiErrors::BadRequest(msg) => {
                write!(f, "{}", msg)
            },
        }
    }
}
