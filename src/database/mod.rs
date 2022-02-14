use postgres::{Client, Error, NoTls};

use crate::tls::tls_config;

pub(crate) struct Psql {
    pub pool: Vec<postgres::Client>,
    url: String,
}

impl Psql {
    pub(crate) fn new(pool_size: usize, url: String) -> Result<Self, Error> {
        let mut pool = Vec::with_capacity(pool_size);
        let config_tls = tls_config();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config_tls);
        pool.iter_mut().for_each(|x| {
            *x = Client::connect(&url, tls.clone()).expect("Could not make connection");
        });
        Ok(Psql { pool, url })
    }
}

#[cfg(test)]
mod test {

    use self::super::*;

    #[tokio::test]
    async fn test_dbconnection() {
        let pool_size = 2;
        let conn = Psql::new(pool_size, "...".into()).unwrap();
        // Check the pool size is correct
        assert_eq!(pool_size, conn.pool.len());
    }
}
