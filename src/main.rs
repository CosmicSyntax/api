use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand_core::OsRng;
use sqlx::Transaction;
use std::error::Error;
// use std::time::SystemTime;
//
// use api::database::models::Customers;
// use api::database::Manager;
// use api::Configuration;
// use chrono::Utc;
use sqlx::postgres::PgPoolOptions;
// use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:admin@localhost:5432/api")
        .await
        .unwrap();
    let x = sqlx::migrate!("./migrations/");
    let _ = x.run(&pool).await;

    let mut tx = pool.begin().await?;
    let z = sqlx::query!(
        "
            INSERT INTO customers(uuid, created_at, updated_at)
            VALUES ($1, $2, $2) RETURNING id;
        ",
        uuid::Uuid::new_v4(),
        chrono::Utc::now(),
    );
    let id = z.fetch_all(&mut tx).await?;
    tx.commit().await?;

    const PASSWORD: &str = "!Q@W#E$R%T^Y&U";
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let x = argon.hash_password(PASSWORD.as_bytes(), &salt).unwrap().to_string();

    let z = sqlx::query!(
        "
            INSERT INTO information(id, username, pw)
            VALUES ($1, $2, $3);
        ",
        id[0].id,
        "myhomie",
        x.as_bytes(),
    );
    z.fetch_all(&pool).await?;

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
    // println!("{}", diff.num_seconds());
    Ok(())
}
