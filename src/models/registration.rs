use std::error::Error;

use actix_web::web::Data;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use serde::Deserialize;
use tracing::error;
use uuid::Uuid;

use crate::db::DB;
use crate::error::{
    ApiErrors, BAD_REQUEST_ERROR, DB_ERROR, DB_ERROR_USER_EXISTS, PW_ERROR, PW_ERROR_INCORRECT,
};

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
        let record = row.fetch_all(&db.pg).await.map_err(Self::db_error_handle)?;

        if !record.is_empty() {
            error!("username already exists in the database");
            return Err(DB_ERROR_USER_EXISTS);
        }

        Ok(())
    }
    /// Adds the user into the DB. The username is stored in plain text, but the password is hashed
    /// using the Argon2 algo.
    pub async fn register(&self, db: &Data<DB>) -> Result<Uuid, ApiErrors> {
        let mut tx = db.pg.begin().await.map_err(Self::db_error_handle)?;
        let row = sqlx::query!(
            "
                INSERT INTO customers(uuid)
                VALUES ($1) RETURNING id, uuid;
            ",
            uuid::Uuid::new_v4(),
        );
        let record = row
            // In 0.7, `Transaction` can no longer implement `Executor` directly,
            // so it must be dereferenced to the internal connection type.
            .fetch_one(&mut *tx)
            .await
            .map_err(Self::db_error_handle)?;

        // TODO: this could be an expensive op... move out of this hotpath
        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let pw_result = argon.hash_password(self.password.as_bytes(), &salt);

        if pw_result.is_err() {
            tx.rollback().await.map_err(Self::db_error_handle)?;
            return Err(BAD_REQUEST_ERROR);
        }

        let pw_result = pw_result.unwrap().to_string();

        let row = sqlx::query!(
            "
                INSERT INTO information(id, username, pw)
                VALUES ($1, $2, $3);
            ",
            record.id,
            self.username,
            pw_result.as_bytes(),
        );
        row.execute(&mut *tx).await.map_err(Self::db_error_handle)?;
        tx.commit().await.map_err(Self::db_error_handle)?;
        Ok(record.uuid)
    }

    pub async fn verify(&self, db: &Data<DB>) -> Result<(), ApiErrors> {
        let row = sqlx::query!(
            "
                SELECT * FROM information WHERE username = $1
            ",
            self.username,
        );
        let record = row.fetch_one(&db.pg).await.map_err(Self::db_error_handle)?;
        let record_pw = String::from_utf8_lossy(&record.pw);
        let parsed_pw = PasswordHash::new(&record_pw).map_err(Self::pw_error_handle)?;
        if Argon2::default()
            .verify_password(self.password.as_bytes(), &parsed_pw)
            .is_ok()
        {
            return Ok(());
        }
        Err(Self::pw_error_handle(PW_ERROR_INCORRECT))
    }

    fn pw_error_handle(e: impl Error) -> ApiErrors {
        error!("{}", e);
        PW_ERROR
    }

    fn db_error_handle(e: sqlx::Error) -> ApiErrors {
        error!("{}", e);
        DB_ERROR
    }
}
