use anyhow::{Ok, Result};
use api::logger::Logger;
use api::{web, global};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:admin@localhost:5432/api")
    //     .await
    //     .unwrap();
    // let x = sqlx::migrate!("./migrations/");
    // let _ = x.run(&pool).await;
    //
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

    Logger::start(Level::INFO);
    global::init_once();
    web::start_server(true).await?;

    Ok(())
}
