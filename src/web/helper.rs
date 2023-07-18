use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    http::StatusCode,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web::{BytesMut, Data},
    HttpResponse,
};
use serde_json::json;
use tracing::{error, span, Instrument, Level};

use crate::{
    db::DB,
    error::{ApiErrors, BAD_REQUEST_ERROR},
    models::registration::UserLogin,
};

pub fn custom_404_handle() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, |res| {
        let request = res.into_parts().0;
        Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
            request,
            {
                HttpResponse::NotFound()
                    .json(json!({
                        "error": "Not Found",
                        "message": "Check the uri and try again",
                    }))
                    .map_into_left_body()
            },
        )))
    })
}

pub async fn verify(data: BytesMut, db: &Data<DB>) -> Result<(), ApiErrors> {
    match serde_json::from_slice::<UserLogin>(&data) {
        Ok(r) => {
            async {
                // check first if the username already exists
                r.verify(db).await
            }
            .instrument(span!(
                Level::ERROR,
                "User verification",
                username = r.username,
            ))
            .await?;
            Ok(())
        }
        Err(e) => {
            error!("{}", e);
            Err(BAD_REQUEST_ERROR)
        }
    }
}
