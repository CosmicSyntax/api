use std::future;
use std::str::FromStr;

use actix_web::web::{BytesMut, Data, Payload};
use actix_web::{get, http::StatusCode, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::StreamExt;
use serde_json::json;
use tracing::{span, Instrument, Level};
use uuid::Uuid;

use crate::db::DB;
use crate::error::ApiErrors;
use crate::global;
use crate::web::helper::verify;
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
            .json(json!({ "message": format!("{e:?}") })),
    }
}

#[get("/token/{id}")]
async fn token(
    pl: Payload,
    path: web::Path<String>,
    db: Data<DB>,
) -> Result<HttpResponse, ApiErrors> {
    let mut data = BytesMut::new();
    pl.for_each(|v| {
        if let Ok(v) = v {
            data.extend_from_slice(&v);
        }
        future::ready(())
    })
    .await;

    verify(data, &db)
        .instrument(span!(Level::ERROR, "User verification",))
        .await?;

    let uuid_string = path.into_inner();
    let uuid = Uuid::from_str(&uuid_string).unwrap();
    let key = global::CONFIG.get().unwrap();
    let token = jwt::get_token(&key.encoder, 10, uuid).unwrap();
    Ok(HttpResponse::build(StatusCode::OK).json(json!({ "token": token })))
}

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(auth).service(token));
}
