use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use rustls::Certificate;

// TLS helper funtions
#[inline(always)]
fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    rustls_pemfile::certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

#[inline(always)]
pub(super) fn tls_config() -> rustls::ClientConfig {
    let mut root = rustls::RootCertStore::empty();
    let certs = load_certs(Path::new("./certs/server.crt")).unwrap();
    root.add(&certs[0]).unwrap();
    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root)
        .with_no_client_auth()
}
