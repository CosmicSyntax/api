use actix_web::web::Data;
use anyhow::{Ok, Result};
use api::db::DB;
use api::error::MANAGER_START_ERROR;
use api::logger::Logger;
use api::{global, web};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {

    Logger::start(Level::INFO);
    global::init_once();

    let db = DB::new(&global::CONFIG.get().ok_or(MANAGER_START_ERROR)?.db_url).await;
    let db = Data::new(db);

    web::start_server(true, db).await?;

    Ok(())
}
