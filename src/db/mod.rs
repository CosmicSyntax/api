use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct DB(pub Pool<Postgres>);

impl DB {
    pub async fn new(addr: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(addr)
            .await
            .expect("Could not make connection to DB");
        Self(pool)
    }
}
