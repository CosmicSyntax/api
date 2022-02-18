use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _conf = api::Configuration::new("./configs/api.yml").await?;

    Ok(())
}
