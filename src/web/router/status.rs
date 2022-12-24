use std::str::FromStr;
use std::time::{Duration, SystemTime};

use actix_web::web;
use actix_web::{get, http::StatusCode, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;
use tokio::{spawn, time::sleep};
use tracing::{info, instrument, span, Instrument, Level};
use uuid::Uuid;

use crate::global;
use crate::web::jwt::{self, validate_token};

#[get("/status")]
#[instrument]
async fn status() -> HttpResponse {
    info!("I am starting");

    spawn(
        async {
            sleep(Duration::from_secs(5)).await;
            info!("I am done!");
        }
        .instrument(span!(
            Level::INFO,
            "status... inside",
            time = format!("{:?}", SystemTime::now())
        )),
    );

    let json_rep = json!({"api": "ok", "db": "ok"});
    HttpResponse::build(StatusCode::OK).json(json_rep)
}

#[get("/testAuth")]
async fn auth(auth: BearerAuth) -> HttpResponse {
    let key = global::CONFIG.get().unwrap();
    let claim = validate_token(auth.token(), &key.decoder, &key.validation);
    match claim {
        Ok(c) => HttpResponse::build(StatusCode::OK).json(json!({
            "message": "ok"
        })),
        Err(e) => HttpResponse::build(StatusCode::UNAUTHORIZED)
            .json(json!({ "message": format!("{:?}", e) })),
    }
}

#[get("/testToken/{id}")]
async fn token(path: web::Path<String>) -> HttpResponse {
    let uuid_string = path.into_inner();
    let uuid = Uuid::from_str(&uuid_string).unwrap();
    let key = global::CONFIG.get().unwrap();
    let token = jwt::get_token(&key.encoder, 10, uuid).unwrap();
    HttpResponse::build(StatusCode::OK).json(json!({ "token": token }))
}

pub fn config_status(cfg: &mut web::ServiceConfig) {
    cfg.service(status).service(auth).service(token);
}
