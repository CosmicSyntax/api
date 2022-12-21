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

    let z = sqlx::query!(
        "
            INSERT INTO customers(uuid, created_at, updated_at)
            VALUES ($1, $2, $2);
        ",
        uuid::Uuid::new_v4(),
        chrono::Utc::now(),
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
