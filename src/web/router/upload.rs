use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use tracing::{instrument, warn};

use crate::{db::DB, error::ApiErrors};

#[post("/unsecure")]
#[instrument(skip(mp, db), level = "error")]
async fn upload(
    mut mp: Multipart,
    req: HttpRequest,
    db: Data<DB>,
) -> Result<HttpResponse, ApiErrors> {
    while let Ok(Some(mut field)) = mp.try_next().await {
        if let Some(file_type) = field.content_disposition().get_filename() {
            warn!("{}", file_type);
        } else {
            return Err(ApiErrors::UploadError(String::from("filename error")))
        }
        while let Some(dat) = field.next().await {
            let _data = dat?;
        }
    }
    Ok(HttpResponse::Ok().json(json!({"message": "Upload success"})))
}

pub fn config_upload(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/upload").service(upload));
}
