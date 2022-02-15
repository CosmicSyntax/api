
use chrono::{DateTime, Local};
use sql_builder::{SqlBuilder, quote};
use uuid::Uuid;

use crate::database;

pub struct Customers {
    uuid: Uuid,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    // id: u32, <-- auto incremented
}

impl Customers {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        let time = chrono::Local::now();
        Customers { uuid, created_at: time, updated_at: time }
    }
}

impl database::DbExec for Customers {
    fn set(&self) -> String {
        let query = SqlBuilder::insert_into("customers")
            .field("uuid")
            .field("created_at")
            .field("updated_at")
            .values(&[&quote(self.uuid.to_string()), &quote(self.created_at.to_string()), &quote(self.updated_at.to_string())])
            .sql();
        query.unwrap()
    }
}
