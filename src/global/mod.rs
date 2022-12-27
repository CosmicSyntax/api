use std::fs;

use jsonwebtoken::{DecodingKey, Validation, Algorithm, EncodingKey};
use once_cell::sync::OnceCell;
use tracing::warn;

use crate::config::Config;

pub struct Configuration {
    pub encoder: EncodingKey,
    pub decoder: DecodingKey,
    pub validation: Validation,
    pub db_url: String,
}

pub static CONFIG: OnceCell<Configuration> = OnceCell::new();

pub fn init_once() {
    let private_key = fs::read("certs/private.ec.key").unwrap();
    let private_key = crate::web::jwt::sec1_to_pkcs8(&private_key);
    let encoder = EncodingKey::from_ec_pem(&private_key).unwrap();

    let public_key = fs::read("certs/public-key.pem").unwrap();
    let decoder = DecodingKey::from_ec_pem(&public_key).unwrap();

    let mut validation = Validation::new(Algorithm::ES256);
    validation.leeway = 0;

    let config = Config::new("./configs/api.yml").expect("could not load configuration yaml");
    let db = config.0[0]["db"]["url"].as_str().unwrap();

    if CONFIG
        .set(Configuration {
            encoder,
            decoder,
            validation,
            db_url: db.to_string(),
        })
        .is_err()
    {
        warn!("Configuration already set.");
    }
}
