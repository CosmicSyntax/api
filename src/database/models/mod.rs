use std::sync::Arc;

use parking_lot::Mutex;
use sql_builder::{quote, SqlBuilder};
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio_postgres::{Client, Error};
use uuid::Uuid;

use crate::database;

#[derive(Debug)]
pub struct Customers {
    uuid: Uuid,
    // created_at: DateTime<Local>, <-- auto generated
    // updated_at: DateTime<Local>,
    // id: u32, <-- auto incremented
}

impl Customers {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Customers { uuid }
    }
}

impl Default for Customers {
    fn default() -> Self {
        Self::new()
    }
}

impl database::DbExec for Customers {
    fn set(&self, client: Arc<Mutex<Option<Client>>>) -> Result<JoinHandle<()>, Error> {
        let uuid = self.uuid.to_string();
        Ok(spawn(async move {
            // take the client out...
            let mut c = client.lock().take().unwrap();
            // start a transaction
            let t = c.transaction().await.unwrap();
            let query = SqlBuilder::insert_into("customers")
                .field("uuid")
                .values(&[quote(uuid)])
                .returning_id()
                .sql();
            let s = query.unwrap();
            let row = t.query_one(&s, &[]).await.unwrap();
            let query = SqlBuilder::insert_into("entries")
                .field("id")
                .field("content")
                .values(&[&row.get::<_, i32>(0).to_string()[..], "'Hello World!!!!'"])
                .sql();
            let s = query.unwrap();
            t.execute(&s, &[]).await.unwrap();
            // end transaction
            t.commit().await.unwrap();
            *client.lock() = Some(c);
        }))
    }
}
