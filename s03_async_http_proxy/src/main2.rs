

use std::net::ToSocketAddrs;

use tokio::{io, net::TcpListener};
use s03_async_http_proxy::http_proxy;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = ("127.0.0.1", 1081u16)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    // 监听TCP连接
    let listener = TcpListener::bind(&addr).await?;
    loop {
        if let Ok((stream, _peer_addr)) = listener.accept().await {
            tokio::spawn(async move {
                if let Err(_err) = http_proxy::handle(stream).await {
                    // eprintln!("{:?}", _err);
                }
            });
        }
    }
}
