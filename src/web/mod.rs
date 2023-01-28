use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};

use crate::db::DB;
mod helper;
pub mod jwt;
mod midware;
mod router;
mod tls;

pub async fn start_server(with_tls: bool, db: Data<DB>) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(helper::custom_404_handle())
            .wrap(middleware::NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(db.clone())
            .configure(router::config_status)
            .configure(router::config_auth)
            .configure(router::config_reg)
    });
    if with_tls {
        server
            .bind_rustls(
                ("0.0.0.0", 8080),
                tls::load_certs("certs/cert.pem", "certs/key.pem"),
            )?
            .run()
            .await
    } else {
        server.bind(("0.0.0.0", 8080))?.run().await
    }
}
