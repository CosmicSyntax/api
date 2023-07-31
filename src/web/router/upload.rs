use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, Data, ReqData},
    HttpRequest, HttpResponse,
};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use tracing::{instrument, warn};

use crate::{
    db::DB,
    error::ApiErrors,
    web::midware::{Msg, UploadAuth},
};

#[post("/insecure")]
#[instrument(skip(mp, db), level = "error")]
async fn upload(
    mut mp: Multipart,
    req: HttpRequest,
    msg: Option<ReqData<Msg>>,
    db: Data<DB>,
) -> Result<HttpResponse, ApiErrors> {

    let Msg(m) = msg.unwrap().into_inner();
    warn!("{}", m);

    while let Ok(Some(mut field)) = mp.try_next().await {
        if let Some(file_type) = field.content_disposition().get_filename() {
            warn!("{}", file_type);
        } else {
            return Err(ApiErrors::UploadError(String::from("filename error")));
        }
        while let Some(dat) = field.next().await {
            let data = dat?;
            warn!("{:?}", data.to_vec());
        }
    }
    let _ = db.pg.size();
    Ok(HttpResponse::Ok().json(json!({"message": "Upload success"})))
}

pub fn config_upload(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/upload").wrap(UploadAuth).service(upload));
}
