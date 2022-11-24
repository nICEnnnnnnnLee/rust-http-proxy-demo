use regex::Regex;
use tokio::io::{self, copy, split, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

lazy_static::lazy_static! {
    static ref REG_HEAD :Regex  = Regex::new(r"(CONNECT|Host:) ([^ :\r\n]+)(?::(\d+))?").unwrap();
}

pub async fn handle(stream: TcpStream) -> io::Result<()> {
    let (mut local_reader, mut local_writer) = split(stream);
    // 读取头部
    let mut head = [0u8; 2048];
    let n = local_reader.read(&mut head[..]).await?;

    let head_str = std::str::from_utf8(&head[..n])
        .map_err(|x| io::Error::new(io::ErrorKind::Interrupted, x))?;

    if let Some(caps) = REG_HEAD.captures(head_str) {
        let host = &caps[2];
        let port = caps.get(3).map_or("80", |m| m.as_str());
        println!("{} {}", host, port);
        // 以下是直连
        let dst_addr = format!("{}:{}", host, port);
        let remote_stream = TcpStream::connect(dst_addr).await?;
        let (mut remote_reader, mut remote_writer) = split(remote_stream);

        if head_str.starts_with("CONNECT") {
            local_writer
                .write_all("HTTP/1.1 200 Connection Established\r\n\r\n".as_bytes())
                .await?;
        } else {
            remote_writer.write_all(&head[..n]).await?;
        }

        let client_to_server = async {
            copy(&mut local_reader, &mut remote_writer).await?;
            remote_writer.shutdown().await
        };

        let server_to_client = async {
            copy(&mut remote_reader, &mut local_writer).await?;
            local_writer.shutdown().await
        };

        tokio::try_join!(client_to_server, server_to_client)?;
    }
    Ok(())
}
