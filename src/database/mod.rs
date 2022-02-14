use postgres::{Client, NoTls, Error};

pub(crate) struct Psql {
    pub pool: Vec<postgres::Client>,
    url: String,
}

impl Psql {
    pub(crate) fn new(pool_size: usize, url: String) -> Result<Self, Error> {
        let mut pool = Vec::with_capacity(pool_size);
        for i in 0..pool_size{
            pool[i] = Client::connect(&url, NoTls)?;
        }
        Ok(
            Psql {
                pool,
                url,
            }
        )
    }
}

#[cfg(test)]
mod test {

    use self::super::*;

    #[tokio::test]
    async fn test_dbconnection() {
        let pool_size = 2;
        let conn = Psql::new(pool_size, "...".into());
        // Check the pool size is correct
        assert_eq!(pool_size, conn.pool.len());
    }
}
