use actix_web::middleware::Logger;
use actix_web::{middleware, App, HttpServer};
mod helper;
pub mod jwt;
mod router;
mod midware;

use crate::tls;

pub async fn start_server(with_tls: bool) -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(helper::custom_404_handle())
            .wrap(middleware::NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(midware::ApiMiddle)
            .configure(router::config_status)
            .configure(router::config_auth)
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
