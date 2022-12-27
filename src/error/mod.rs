use std::error::Error;
use std::fmt::Display;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError, body::BoxBody,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiErrors {
    ServerStartError(&'static str),
    BadRequest(&'static str),
}

pub const MANAGER_START_ERROR: ApiErrors = ApiErrors::ServerStartError("could not start server");
pub const BAD_REQUEST_ERROR: ApiErrors = ApiErrors::BadRequest("request is not correct");

impl Error for ApiErrors {}

impl ResponseError for ApiErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiErrors::ServerStartError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut res = HttpResponse::build(self.status_code());
        res.insert_header(ContentType::json());

        res.body(BoxBody::new(json!({
            "error": self.to_string(),
        }).to_string()))
    }
}

impl Display for ApiErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
