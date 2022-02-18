use std::error::Error;
use std::sync::Arc;

use api::database::Psql;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conf = api::Configuration::new("./configs/api.yml").await?;
    let conn1 = Psql::new(20, conf.0[0]["db"]["url_host"].as_str().unwrap().to_string()).await?;
    let saf1 = Arc::new(conn1);


    let start = Utc::now();

    let handles1: Vec<_> = (0..2000)
        .map(|_| {
            let conn = Arc::clone(&saf1);
            let customer = api::database::models::Customers::new();
            tokio::spawn(async move {
                conn.set(&customer).await;
            })
        }).collect();

    for handle in handles1 {
        handle.await.unwrap();
    }

    let stop = Utc::now();
    println!("{}", (stop - start).num_milliseconds());
    
    Ok(())
}
