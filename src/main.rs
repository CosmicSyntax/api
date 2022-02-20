use std::error::Error;

use api::database::models::Customers;
use api::database::Manager;
use api::Configuration;
use chrono::Utc;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conf = Configuration::new("./configs/api.yml").await?;
    let (s, r) = channel::<Customers>(10000);
    let mut manager = Manager::new(r, 50, conf.0[0]["db"]["url_host"].as_str().unwrap()).await;
    let r = manager.start()?;

    let start = Utc::now().time();
    println!("GO!");
    for _ in 0..10000 {
        let customer: Customers = Default::default();
        s.send(customer).await?;
    }
    let stop = Utc::now().time();
    let diff = stop - start;
    println!("{}", diff.num_seconds());
    r.await?;
    Ok(())
}
