use std::error::Error;

use actix_web::web::Data;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use serde::Deserialize;

use crate::db::DB;
use crate::error::BAD_REQUEST_ERROR;

#[derive(Deserialize)]
pub struct UserLogin<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> UserLogin<'a> {
    /// Adds the user into the DB. The username is stored in plain text, but the password is hashed
    /// using the Argon2 algo.
    pub async fn register(&self, db: Data<DB>) -> Result<(), Box<dyn Error>> {
        let mut tx = db.0.begin().await?;
        let row = sqlx::query!(
            "
                INSERT INTO customers(uuid)
                VALUES ($1) RETURNING id, uuid;
            ",
            uuid::Uuid::new_v4(),
        );
        let record = row.fetch_one(&mut tx).await?;

        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let pw_result = argon.hash_password(self.password.as_bytes(), &salt);

        if pw_result.is_err() {
            tx.rollback().await?;
            return Err(Box::new(BAD_REQUEST_ERROR));
        }

        let row = sqlx::query!(
            "
                INSERT INTO information(id, username, pw)
                VALUES ($1, $2, $3);
            ",
            record.id,
            self.username,
            self.password.as_bytes(),
        );
        row.execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }
}
