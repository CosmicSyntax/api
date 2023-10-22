use std::{future, str::FromStr};

use actix_web::{
    cookie::Cookie,
    get,
    http::StatusCode,
    web::{self, BytesMut, Data, Payload},
    HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::StreamExt;
use serde_json::json;
use tracing::{error, instrument, span, Instrument, Level};
use uuid::Uuid;

use crate::{
    db::DB,
    error::ApiErrors,
    global,
    web::{
        helper::verify,
        jwt::{self, validate_token},
    },
};

#[get("/check")]
#[instrument(level = "error")]
async fn auth(auth: BearerAuth) -> HttpResponse {
    let key = global::CONFIG.get().unwrap();
    let claim = validate_token(auth.token(), &key.decoder, &key.validation);
    match claim {
        Ok(_) => HttpResponse::build(StatusCode::OK).json(json!({
            "message": "ok"
        })),
        Err(e) => {
            error!("{e}");
            HttpResponse::build(StatusCode::UNAUTHORIZED)
                .json(json!({ "message": format!("{e:?}") }))
        }
    }
}

#[get("/token/{id}/{mess}")]
async fn token(
    pl: Payload,
    path: web::Path<(String, String)>,
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

    let path = path.into_inner();
    let uuid = Uuid::from_str(&path.0).unwrap();
    let key = global::CONFIG.get().unwrap();
    let token = jwt::get_token(&key.encoder, 10, uuid).unwrap();
    Ok(HttpResponse::build(StatusCode::OK).json(json!({ "token": token, "message": &path.1 })))
}

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(auth).service(token));
}

#[cfg(test)]
mod test {
    use actix_web::{http::header, test, web::Data, App};
    use serde::Deserialize;
    use serde_json::json;
    use sqlx::PgPool;

    use crate::{db::DB, global, models::registration::UserLogin, web::router::config_auth};

    #[sqlx::test]
    async fn test_token_and_check(pool: PgPool) {
        global::init_once();

        let data = Data::new(DB { pg: pool });

        const USERNAME: &str = "dtchoi";
        const PW: &str = "12345678";

        // create user profile
        let user = UserLogin {
            username: USERNAME,
            password: PW,
        };

        // Setup data
        let uid = user.register(&data).await.unwrap();

        // Test with wrong creds
        let app = test::init_service(App::new().app_data(data).configure(config_auth)).await;
        let req = test::TestRequest::get()
            .uri("/auth/token/00000000-0000-0000-0000-000000000000/abd")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Test with right creds
        let req = test::TestRequest::get()
            .uri(&format!("/auth/token/{}/abd", uid))
            .set_payload(
                json!({
                    "username": "dtchoi",
                    "password": "12345678",
                })
                .to_string(),
            )
            .to_request();

        #[derive(Deserialize)]
        struct Message {
            message: String,
            token: String,
        }

        let resp: Message = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.message, "abd");
        let token = resp.token;
        assert!(!token.is_empty());

        // Test check with right token
        let req = test::TestRequest::get()
            .uri("/auth/check")
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Test check with wrong token
        let req = test::TestRequest::get()
            .uri("/auth/check")
            .insert_header((header::AUTHORIZATION, "Bearer wrongtokenman"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
