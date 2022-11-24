use s04_async_https_connection_via_tcp::{handle, init, utils::tls};
use std::{io, net::ToSocketAddrs};
use tokio::io::{copy, stdout as tokio_stdout, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = ("127.0.0.1", 443u16)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    // 监听TCP连接
    let listener = TcpListener::bind(&addr).await?;
    let acceptor = init("pixiv.net.crt", "pixiv.net.key")?;
    // 等待3s(此时本地服务已经建立)， 然后发送一个HTTPS 请求
    tokio::spawn(async move {
        sleep(Duration::from_secs(3)).await;
        send_a_https_request().await
    });
    // 处理请求连接
    loop {
        match listener.accept().await {
            Ok((stream, _peer_addr)) => {
                let acceptor = acceptor.clone();
                match acceptor.accept(stream).await {
                    Ok(stream) => {
                        tokio::spawn(async move {
                            if let Err(_err) = handle(stream).await {
                                eprintln!("TLS Handler err: {:?}", _err);
                            }
                        });
                    }
                    Err(_err) => {
                        eprintln!("Tls err: {:?}", _err);
                    }
                }
            }
            Err(_err) => {
                eprintln!("Tcp err: {:?}", _err);
            }
        }
    }
}

async fn send_a_https_request() -> io::Result<()> {
    let allow_insecure = true;
    let sni = "pixiv.net";
    let dst_addr = "127.0.0.1";
    let dst_port = 443;
    let content = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", sni);

    let (mut reader, mut writer) = tls::connect(dst_addr, dst_port, sni, allow_insecure).await?;

    writer.write_all(content.as_bytes()).await?;

    let mut stdout = tokio_stdout();

    // stdout.write_all("\r\nReceived response from server: ".as_bytes()).await?;
    copy(&mut reader, &mut stdout).await?;

    Ok(())
}
