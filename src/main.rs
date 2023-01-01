use actix_web::web::Data;
use anyhow::{Ok, Result};
use api::{db::DB, error::SERVER_START_ERROR, global, logger::Logger, web};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::start(Level::WARN);
    global::init_once();

    let db = DB::new(&global::CONFIG.get().ok_or(SERVER_START_ERROR)?.db_url).await;
    let db = Data::new(db);

    web::start_server(true, db).await?;

    Ok(())
}
