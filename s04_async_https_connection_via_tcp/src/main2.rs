use std::io;
use tokio::io::{copy, stdout as tokio_stdout, AsyncWriteExt};

use s04_async_https_connection_via_tcp::utils::tls;

#[tokio::main]
async fn main() -> io::Result<()> {
    let allow_insecure = true;
    // let sni = "www.baidu.com";
    // let dst_addr = "www.baidu.com";
    let sni = "baidu.com";
    let dst_addr = "baidu.com";
    let dst_port = 443;
    let content = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", sni);

    let (mut reader, mut writer) = tls::connect(dst_addr, dst_port, sni, allow_insecure).await?;

    writer.write_all(content.as_bytes()).await?;

    let mut stdout = tokio_stdout();

    copy(&mut reader, &mut stdout).await?;

    Ok(())
}
