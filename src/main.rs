use std::error::Error;

use api::database::Psql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conf = api::Configuration::new("./configs/api.yml").await?;
    let conn = Psql::new(5, conf.0[0]["db"]["url_host"].as_str().unwrap().to_string()).await?;

    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    conn.set(&api::database::models::Customers::new()).await;
    
    Ok(())
}
