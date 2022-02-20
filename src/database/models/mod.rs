use sql_builder::{quote, SqlBuilder};
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
        Customers {
            uuid,
        }
    }
}

impl Default for Customers {
    fn default() -> Self {
        Self::new()
    }
}

impl database::DbExec for Customers {
    fn set(&self) -> String {
        let query = SqlBuilder::insert_into("customers")
            .field("uuid")
            .values(&[
                &quote(self.uuid.to_string()),
            ])
            .sql();
        query.unwrap()
    }
}
