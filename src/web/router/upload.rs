use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use tracing::instrument;

use crate::{db::DB, error::ApiErrors};

#[post("/unsecure")]
#[instrument(skip(pl, db), level = "error")]
async fn upload(pl: Multipart, req: HttpRequest, db: Data<DB>) -> Result<HttpResponse, ApiErrors> {
    todo!()
}

pub fn config_upload(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/upload").service(upload));
}
