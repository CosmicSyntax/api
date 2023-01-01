use std::future;

use crate::{
    db::DB,
    error::{ApiErrors, BAD_REQUEST_ERROR},
    models::registration::UserLogin,
};
use actix_web::{
    post,
    web::{self, BytesMut, Data, Payload},
    HttpResponse,
};
use futures::StreamExt;
use serde_json::json;
use tracing::{span, Instrument, Level};

#[post("/register")]
async fn register(pl: Payload, db: Data<DB>) -> Result<HttpResponse, ApiErrors> {
    // Collect the paylod from a stream
    let mut data = BytesMut::new();
    pl.for_each(|v| {
        if let Ok(v) = v {
            data.extend_from_slice(&v);
        }
        future::ready(())
    })
    .await;

    // Deseralization
    let registration = serde_json::from_slice::<UserLogin>(&data);

    // Add user to DB
    if let Ok(r) = registration {
        let span = span!(Level::ERROR, "New User Registration", username = r.username,);
        // check first...
        r.check(&db).instrument(span.clone()).await?;
        let uuid = r.register(&db).instrument(span).await?;
        Ok(HttpResponse::Ok().json(json!({"message": uuid.to_string()})))
    } else {
        Err(BAD_REQUEST_ERROR)
    }
}

pub fn config_reg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/membership").service(register));
}

// let mut tx = pool.begin().await?;
// let z = sqlx::query!(
//     "
//         INSERT INTO customers(uuid, created_at, updated_at)
//         VALUES ($1, $2, $2) RETURNING id;
//     ",
//     uuid::Uuid::new_v4(),
//     chrono::Utc::now(),
// );
// let id = z.fetch_all(&mut tx).await?;
// tx.commit().await?;
//
// const PASSWORD: &str = "!Q@W#E$R%T^Y&U";
// let argon = Argon2::default();
// let salt = SaltString::generate(&mut OsRng);
// let x = argon.hash_password(PASSWORD.as_bytes(), &salt).unwrap().to_string();
//
// let z = sqlx::query!(
//     "
//         INSERT INTO information(id, username, pw)
//         VALUES ($1, $2, $3);
//     ",
//     id[0].id,
//     "myhomie",
//     x.as_bytes(),
// );
// z.fetch_all(&pool).await?;
//
// let pw = sqlx::query!(
//     "
//         SELECT * FROM information
//         WHERE id = $1;
//     ",
//     id[0].id,
// );
// let pw = pw.fetch_all(&pool).await?;
// let pws = String::from_utf8_lossy(&pw[0].pw);
// let pw = PasswordHash::new(&pws).unwrap();
//
// println!("{}", pw);
//
// if pw.verify_password(&[&argon], PASSWORD).is_ok() {
//     println!("MATCH!");
// } else {
//     println!("NOT A MATCH!");
// }

// let conf = Configuration::new("./configs/api.yml").await?;
// let (s, r) = channel(10000);
// let mut manager = Manager::new(r, 50, conf.0[0]["db"]["url_host"].as_str().unwrap()).await;
// let r = manager.start()?;
//
// let start = Utc::now().time();
// for _ in 0..10000 {
//     let customer: Customers = Default::default();
//     s.send(Box::new(customer)).await?;
// }
// drop(s);
// r.await?;
// let stop = Utc::now().time();
// let diff = stop - start;
