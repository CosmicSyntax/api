use std::str::FromStr;

use actix_web::{get, http::StatusCode, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;
use uuid::Uuid;

use crate::global;
use crate::web::jwt::{self, validate_token};

#[get("/check")]
async fn auth(auth: BearerAuth) -> HttpResponse {
    let key = global::CONFIG.get().unwrap();
    let claim = validate_token(auth.token(), &key.decoder, &key.validation);
    match claim {
        Ok(_) => HttpResponse::build(StatusCode::OK).json(json!({
            "message": "ok"
        })),
        Err(e) => HttpResponse::build(StatusCode::UNAUTHORIZED)
            .json(json!({ "message": format!("{:?}", e) })),
    }
}

#[get("/token/{id}")]
async fn token(path: web::Path<String>) -> HttpResponse {
    let uuid_string = path.into_inner();
    let uuid = Uuid::from_str(&uuid_string).unwrap();
    let key = global::CONFIG.get().unwrap();
    let token = jwt::get_token(&key.encoder, 10, uuid).unwrap();
    HttpResponse::build(StatusCode::OK).json(json!({ "token": token }))
}

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(auth).service(token));
}
