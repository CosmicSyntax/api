use std::time::{Duration, SystemTime};

use actix_web::cookie::{self, Cookie};
use actix_web::HttpRequest;
use actix_web::{get, http::StatusCode, web, HttpResponse};
use serde_json::json;
use tokio::{spawn, time::sleep};
use tracing::{info, instrument, span, warn, Instrument, Level};

use crate::web::midware::ApiMiddle;

#[get("")]
#[instrument]
async fn status(req: HttpRequest) -> HttpResponse {
    info!("I am starting");
    let mut psession = false;

    let cookie = if let Some(c) = req.cookie("status") {
        psession = true;
        warn!("{:?}", c);
        c
    } else {
        // no cookie... set one
        Cookie::build("status", "good")
            .http_only(true)
            .secure(true)
            .max_age(cookie::time::Duration::new(60, 0))
            .finish()
    };

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

    if psession {
        HttpResponse::build(StatusCode::OK)
            .json(json!({"api": "ok", "db": "ok", "message": "hello again!"}))
    } else {
        HttpResponse::build(StatusCode::OK)
            .cookie(cookie)
            .json(json!({"api": "ok", "db": "ok"}))
    }
}

pub fn config_status(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/status").wrap(ApiMiddle).service(status));
}
