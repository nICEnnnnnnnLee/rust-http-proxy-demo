use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::io::split;
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

use tokio_rustls::rustls::{self, ClientConfig, OwnedTrustAnchor, RootCertStore};

struct NoCertVerifier {}

impl rustls::client::ServerCertVerifier for NoCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

pub async fn connect(
    dst_addr: &str,
    dst_port: u16,
    sni: &str,
    allow_insecure: bool,
) -> io::Result<(
    ReadHalf<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>,
    WriteHalf<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>,
)> {
    let addr = (dst_addr, dst_port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    if allow_insecure {
        config
            .dangerous()
            .set_certificate_verifier(Arc::new(NoCertVerifier {}));
    }

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr).await?;

    let domain = rustls::ServerName::try_from(sni)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?;

    let stream = connector.connect(domain, stream).await?;
    // stream.write_all(content.as_bytes()).await?;

    // let (mut reader, mut writer) = split(stream);

    Ok(split(stream))
}
