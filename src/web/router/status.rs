use std::time::{Duration, SystemTime};

use actix_web::{get, http::StatusCode, web, HttpResponse};
use serde_json::json;
use tokio::{spawn, time::sleep};
use tracing::{info, instrument, span, Instrument, Level};

use crate::web::midware::ApiMiddle;

#[get("")]
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

pub fn config_status(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/status").wrap(ApiMiddle).service(status));
}
