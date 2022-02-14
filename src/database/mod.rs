use tokio_postgres::{Client, Error};

use crate::tls::tls_config;

pub struct Psql {
    pub pool: Vec<Client>,
    url: String,
}

impl Psql {
    pub async fn new(pool_size: usize, url: String) -> Result<Self, Error> {
        let mut pool = Vec::with_capacity(pool_size);
        let config_tls = tls_config();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config_tls);
        for _ in 0..pool_size {
            let (client, conn) = tokio_postgres::connect(&url, tls.clone()).await?;
            pool.push(client);
            tokio::spawn(async move {
                if let Err(e) = conn.await {
                    eprintln!("connection error: {}", e);
                }
            });
        }
        Ok(Psql { pool, url })
    }
}

#[cfg(test)]
mod test {

    use self::super::*;

    // #[tokio::test]
    #[tokio::test]
    async fn test_dbconnection() {
        let pool_size = 2;
        let conn = Psql::new(pool_size, "host=localhost user=postgres password=admin port=5432 sslmode=require".into()).await.unwrap();
        // Check the pool size is correct
        assert_eq!(pool_size, conn.pool.len());
    }
}
