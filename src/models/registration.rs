use actix_web::web::Data;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand_core::OsRng;
use serde::Deserialize;
use tracing::error;
use uuid::Uuid;

use crate::db::DB;
use crate::error::{ApiErrors, BAD_REQUEST_ERROR, DB_ERROR, DB_ERROR_USER_EXISTS};

#[derive(Deserialize)]
pub struct UserLogin<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> UserLogin<'a> {
    // Check if all the prereqs are met before adding the user in the database
    pub async fn check(&self, db: &Data<DB>) -> Result<(), ApiErrors> {
        let row = sqlx::query!(
        "
            SELECT id FROM information WHERE username = $1;
        ",
        self.username,
        );
        let record = row.fetch_all(&db.0).await.map_err(Self::db_error_handle)?;

        if !record.is_empty() {
            error!("username already exists in the database");
            return Err(DB_ERROR_USER_EXISTS);
        }

        Ok(())
    }
    /// Adds the user into the DB. The username is stored in plain text, but the password is hashed
    /// using the Argon2 algo.
    pub async fn register(&self, db: &Data<DB>) -> Result<Uuid, ApiErrors> {
        let mut tx =
            db.0.begin()
                .await
                .map_err(Self::db_error_handle)?;
        let row = sqlx::query!(
            "
                INSERT INTO customers(uuid)
                VALUES ($1) RETURNING id, uuid;
            ",
            uuid::Uuid::new_v4(),
        );
        let record = row
            .fetch_one(&mut tx)
            .await
            .map_err(Self::db_error_handle)?;

        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let pw_result = argon.hash_password(self.password.as_bytes(), &salt);

        if pw_result.is_err() {
            tx.rollback().await.map_err(Self::db_error_handle)?;
            return Err(BAD_REQUEST_ERROR);
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
        row.execute(&mut tx).await.map_err(Self::db_error_handle)?;
        tx.commit().await.map_err(Self::db_error_handle)?;
        Ok(record.uuid)
    }

    fn db_error_handle(e: sqlx::Error) -> ApiErrors {
        error!("{}", e);
        DB_ERROR
    }
}
