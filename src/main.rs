use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conf = api::Configuration::new("./configs/api.yml").await?;
    println!("{:#?}", conf.0);
    Ok(())
}
