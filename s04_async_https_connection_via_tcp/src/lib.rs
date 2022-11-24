pub mod utils;

use rustls_pemfile::{certs, rsa_private_keys};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use tokio::io::{split, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_rustls::rustls::{Certificate, PrivateKey};
use tokio_rustls::TlsAcceptor;

lazy_static::lazy_static! {
    static ref RESPONSE_403:&'static str = concat!(
        "HTTP/1.1 403 Forbidden\r\n" ,"Content-Length: 0\r\n" ,"Connection: closed\r\n\r\n"
    );
    static ref RESPONSE_200:&'static str = concat!(
        "HTTP/1.1 200 OK\r\n" ,
        "Content-Length: 11\r\n" ,
        "Connection: closed\r\n\r\n",
        "Are you OK?",
    );
}

pub fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

pub fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

pub fn init(cert_path: &str, key_path: &str) -> io::Result<TlsAcceptor> {
    let certs = load_certs(Path::new(cert_path))?;
    let mut keys = load_keys(Path::new(key_path))?;

    let server_conf = tokio_rustls::rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .map_err(|_err| io::Error::new(io::ErrorKind::InvalidInput, "TLS cert loading error"))?;
    Ok(TlsAcceptor::from(std::sync::Arc::new(server_conf)))
}

pub async fn handle<IO>(stream: IO) -> io::Result<()>
where
    IO: AsyncRead + AsyncWrite + Unpin + AsyncWriteExt,
{
    let (mut local_reader, mut local_writer) = split(stream);

    // 从头部读取信息
    let mut head = [0u8; 2048];
    let n = local_reader.read(&mut head[..]).await?;
    if n == 2048 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Receive a unexpected big size of header!!",
        ));
    }
    let head_str = std::str::from_utf8(&head[..n])
        .map_err(|x| io::Error::new(io::ErrorKind::Interrupted, x))?;
    println!("\r\nReceived request from client: \r\n{}\r\n", head_str);

    // 回复200OK
    local_writer.write_all(RESPONSE_200.as_bytes()).await?;
    local_writer.shutdown().await?;
    Ok(()) as io::Result<()>
}
